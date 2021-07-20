use crate::{
    material::Material,
    math::{Point3D, Vector3},
};

use super::Triangle;

#[derive(Debug, Clone, PartialEq)]
pub struct Vertex {
    point: Point3D,
    // TODO: calculate normal vectors - MCL - 2021-07-19
    // normal: Vector3,
}

pub struct PolygonMesh {
    faces: usize,
    material: Material,
    face_index: Vec<usize>,
    vertex_index: Vec<usize>,
    vertices: Vec<Vertex>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TriangleMesh {
    num_triangles: usize,
    vertices: Vec<Vertex>,
    triangles: Vec<usize>,
    material: Material,
}

impl TriangleMesh {
    pub fn new(poly: &PolygonMesh) -> Self {
        let mut num_triangles = 0;
        let mut k = 0;
        let mut max_vertex_index = 0;

        for i in 0..poly.faces {
            let face_size = poly.face_index[i];
            num_triangles += face_size - 2;
            for j in 0..face_size {
                if poly.vertex_index[k + j] > max_vertex_index {
                    max_vertex_index = poly.vertex_index[k + j];
                }
            }
            k += face_size;
        }

        let mut triangles = Vec::with_capacity(num_triangles * 3);
        k = 0;
        for i in 0..poly.faces {
            for j in 0..(poly.face_index[i] - 2) {
                triangles.push(poly.vertex_index[k]);
                triangles.push(poly.vertex_index[k + j + 1]);
                triangles.push(poly.vertex_index[k + j + 2]);
            }
            k += poly.face_index[i];
        }

        Self {
            num_triangles,
            vertices: poly.vertices.clone(),
            triangles,
            material: Material::default(),
        }
    }
}
