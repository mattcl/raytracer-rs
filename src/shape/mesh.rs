use std::f64::{INFINITY, NEG_INFINITY};

use crate::{
    error::{RTError, Result},
    geo::GeoMesh,
    material::{Material, TextureCoord, Textured},
    math::{Matrix4, Point2D, Point3D, Vector2, Vector3},
    ply::Ply,
    ray::Ray,
};

use super::{
    triangle::pre_calc_traingle_intersect,
    BoundingBox,
    Intersect,
    Intersection,
    Shape,
    Transformable
};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    pub point: Point3D,
    pub normal: Vector3,
    pub texture_coord: Point2D,
}

impl Vertex {
    pub fn new(point: Point3D, normal: Vector3, texture_coord: Point2D) -> Self {
        Self {
            point,
            normal,
            texture_coord,
        }
    }

    pub fn transform(&mut self, matrix: &Matrix4, t_inv_matrix: &Matrix4) {
        self.point = matrix * self.point;
        self.normal = t_inv_matrix * self.normal;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShadingMode {
    Flat,
    Smooth,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct TriAddr {
    v0: usize,
    v1: usize,
    v2: usize,
    v0v1: Vector3,
    v0v2: Vector3,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TriangleMesh {
    num_triangles: usize,
    vertices: Vec<Vertex>,
    triangles: Vec<TriAddr>,
    triangle_normals: Vec<Vector3>,
    bounding_box: BoundingBox,
    material: Material,
    wto: Matrix4,
    otw: Matrix4,
    shading_mode: ShadingMode,
}

impl TriangleMesh {
    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }

    pub fn with_shading(mut self, mode: ShadingMode) -> Self {
        self.shading_mode = mode;
        self
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn triangle_vertices(&self, index: usize) -> Option<(&Vertex, &Vertex, &Vertex)> {
        let t = self.triangles.get(index)?;
        Some((
            self.vertices.get(t.v0)?,
            self.vertices.get(t.v1)?,
            self.vertices.get(t.v2)?,
        ))
    }
}

impl Intersect for TriangleMesh {
    fn intersect<'a>(&self, ray: &Ray, shape_ref: &'a Shape) -> Option<Intersection<'a>> {
        if self.bounding_box.intersect(ray, shape_ref).is_some() {
            if let Some((intersect, triangle_index)) = (0..self.num_triangles)
                .filter_map(|i| {
                    self.triangles
                        .get(i)
                        .and_then(|tri| {
                            pre_calc_traingle_intersect(
                                &self.vertices[tri.v0].point,
                                &tri.v0v1,
                                &tri.v0v2,
                                ray,
                            )
                        })
                        .and_then(|intersect| Some((intersect, i)))
                })
                .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            {
                // this is safe since we know it must exist for us to be here
                let (v0, v1, v2, ..) = self.triangle_vertices(triangle_index).unwrap();
                let (dist, uv) = intersect;

                let hit_coord = (1.0 - uv.x() - uv.y()) * Vector2::from(v0.texture_coord)
                    + uv.x() * Vector2::from(v1.texture_coord)
                    + uv.y() * Vector2::from(v2.texture_coord);

                let point = ray.point_at(dist);

                // face normal
                let normal = match self.shading_mode {
                    ShadingMode::Flat => *self.triangle_normals.get(triangle_index)?,
                    ShadingMode::Smooth => {
                        (1.0 - uv.x() - uv.y()) * v0.normal
                            + uv.x() * v1.normal
                            + uv.y() * v2.normal
                    }
                };

                let intersect = Intersection::new(dist, shape_ref)
                    .location(point)
                    .normal(normal)
                    .texture_coord(TextureCoord::new(hit_coord.into(), self.material.scale));

                return Some(intersect);
            }
        }

        None
    }

    fn normal_at(&self, _point: &Point3D) -> Option<Vector3> {
        panic!("Normals for triangle meshes should be accessed via the intersection");
    }
}

impl Textured for TriangleMesh {
    fn texture_coord(&self, _point: &Point3D) -> TextureCoord {
        panic!("TextureCoord for triangle meshes should be accessed via the intersection");
    }
}

impl Transformable for TriangleMesh {
    fn transform(&mut self, matrix: &Matrix4) -> Result<()> {
        self.wto = matrix.inverse().ok_or_else(|| {
            RTError::Error(format!(
                "Cannot transform with matrix {:#?} as it has no inverse",
                matrix
            ))
        })?;
        self.otw = matrix.clone();

        let transpose = self.wto.transpose();

        self.vertices
            .iter_mut()
            .for_each(|v| v.transform(matrix, &transpose));

        let mut min = [INFINITY; 3];
        let mut max = [NEG_INFINITY; 3];
        for vertex in &self.vertices {
            for i in 0..3 {
                let v = vertex.point[i];
                if v < min[i] {
                    min[i] = v;
                }

                if v > max[i] {
                    max[i] = v;
                }
            }
        }

        // recompute our cache
        for t in self.triangles.iter_mut() {
            t.v0v1 = self.vertices[t.v1].point - self.vertices[t.v0].point;
            t.v0v2 = self.vertices[t.v2].point - self.vertices[t.v0].point;
        }

        self.bounding_box = BoundingBox::new(min.into(), max.into());

        Ok(())
    }

    fn object_to_world(&self) -> Option<&Matrix4> {
        Some(&self.otw)
    }

    fn world_to_object(&self) -> Option<&Matrix4> {
        Some(&self.wto)
    }
}

impl From<TriangleMesh> for Shape {
    fn from(t: TriangleMesh) -> Self {
        Shape::TriangleMesh(t)
    }
}

impl From<GeoMesh> for TriangleMesh {
    fn from(geo: GeoMesh) -> Self {
        let mut num_triangles = 0;
        let mut k = 0;
        let mut max_vertex_index = 0;

        for i in 0..geo.num_faces {
            let face_size = geo.face_index[i];
            num_triangles += face_size - 2;
            for j in 0..face_size {
                if geo.vertex_index[k + j] > max_vertex_index {
                    max_vertex_index = geo.vertex_index[k + j];
                }
            }
            k += face_size;
        }

        let mut triangles = Vec::with_capacity(num_triangles);
        let mut triangle_normals = Vec::with_capacity(num_triangles);
        k = 0;
        for i in 0..geo.num_faces {
            for j in 0..(geo.face_index[i] - 2) {
                let v0 = geo.vertex_index[k];
                let v1 = geo.vertex_index[k + j + 1];
                let v2 = geo.vertex_index[k + j + 2];

                let v0v1 = geo.vertices[v1].point - geo.vertices[v0].point;
                let v0v2 = geo.vertices[v2].point - geo.vertices[v0].point;

                triangles.push(TriAddr { v0, v1, v2, v0v1, v0v2 });
                triangle_normals.push(geo.face_normals[i]);
            }

            k += geo.face_index[i];
        }

        Self {
            num_triangles,
            vertices: geo.vertices,
            triangles,
            triangle_normals,
            bounding_box: geo.bounding_box,
            material: Material::default(),
            wto: Matrix4::I,
            otw: Matrix4::I,
            shading_mode: ShadingMode::Flat,
        }
    }
}

impl From<Ply> for TriangleMesh {
    fn from(ply: Ply) -> Self {
        let mut num_triangles = 0;
        let mut max_vertex_index = 0;

        for i in 0..ply.faces.len() {
            let face_size = ply.faces[i].len();
            num_triangles += face_size - 2;
            for j in 0..face_size {
                if ply.faces[i][j] > max_vertex_index {
                    max_vertex_index = ply.faces[i][j];
                }
            }
        }

        let mut triangles = Vec::with_capacity(num_triangles);
        let mut triangle_normals = Vec::with_capacity(num_triangles);
        for i in 0..ply.faces.len() {
            for j in 0..ply.faces[i].len() - 2 {
                let v0 = ply.faces[i][0];
                let v1 = ply.faces[i][j + 1];
                let v2 = ply.faces[i][j + 2];

                let v0v1 = ply.vertices[v1].point - ply.vertices[v0].point;
                let v0v2 = ply.vertices[v2].point - ply.vertices[v0].point;

                triangles.push(TriAddr { v0, v1, v2, v0v1, v0v2 });
                triangle_normals.push(ply.face_normals[i]);
            }
        }

        Self {
            num_triangles,
            vertices: ply.vertices,
            triangles,
            triangle_normals,
            bounding_box: ply.bounding_box,
            material: Material::default(),
            wto: Matrix4::I,
            otw: Matrix4::I,
            shading_mode: ShadingMode::Flat,
        }
    }
}

impl TriangleMesh {
    pub fn partition(&mut self) {
        let mut min = [INFINITY; 3];
        let mut max = [NEG_INFINITY; 3];
        for vertex in &self.vertices {
            for i in 0..3 {
                let v = vertex.point[i];
                if v < min[i] {
                    min[i] = v;
                }

                if v > max[i] {
                    max[i] = v;
                }
            }
        }


        // let mid = (max - min) / 2.0  + min;
    }
}
