use std::{
    collections::HashMap,
    convert::TryFrom,
    f64::{INFINITY, NEG_INFINITY},
    num::{ParseFloatError, ParseIntError},
};

use crate::{
    error::{RTError, Result},
    math::{Point2D, Point3D, Vector3},
    shape::{mesh::Vertex, BoundingBox},
};

#[derive(Debug, Clone)]
pub struct GeoMesh {
    pub num_faces: usize,
    pub face_index: Vec<usize>,
    pub vertex_index: Vec<usize>,
    pub vertices: Vec<Vertex>,
    pub face_normals: Vec<Vector3>,
    pub bounding_box: BoundingBox,
}

impl TryFrom<&str> for GeoMesh {
    type Error = RTError;

    fn try_from(s: &str) -> Result<Self> {
        let mut iter = s.lines();
        let num_faces: usize = iter
            .next()
            .ok_or_else(|| RTError::InvalidGeo(s.into()))?
            .trim()
            .parse()?;

        let face_index = iter
            .next()
            .ok_or_else(|| RTError::InvalidGeo(s.into()))?
            .split(' ')
            .map(|ch| ch.parse())
            .collect::<std::result::Result<Vec<usize>, ParseIntError>>()?;

        let vertex_index = iter
            .next()
            .ok_or_else(|| RTError::InvalidGeo(s.into()))?
            .split(' ')
            .map(|ch| ch.parse())
            .collect::<std::result::Result<Vec<usize>, ParseIntError>>()?;

        let vertex_pos = iter
            .next()
            .ok_or_else(|| RTError::InvalidGeo(s.into()))?
            .split(' ')
            .map(|ch| ch.parse())
            .collect::<std::result::Result<Vec<f64>, ParseFloatError>>()?
            .chunks(3)
            .map(|chunk| Point3D::try_from(Vec::from(chunk)))
            .collect::<Result<Vec<Point3D>>>()?;

        let mut min = [INFINITY; 3];
        let mut max = [NEG_INFINITY; 3];
        for point in &vertex_pos {
            for i in 0..3 {
                let v = point[i];
                if v < min[i] {
                    min[i] = v;
                }

                if v > max[i] {
                    max[i] = v;
                }
            }
        }

        let bounding_box = BoundingBox::new(min.into(), max.into());

        // So I'm convinced the scratchapixel test.geo has the wrong values
        // for the normals so we're just going to ignore this and derive
        // them ourselves
        iter.next().ok_or_else(|| RTError::InvalidGeo(s.into()))?;

        let tex_coords = iter
            .next()
            .ok_or_else(|| RTError::InvalidGeo(s.into()))?
            .split(' ')
            .map(|ch| ch.parse())
            .collect::<std::result::Result<Vec<f64>, ParseFloatError>>()?
            .chunks(2)
            .map(|chunk| Point2D::try_from(Vec::from(chunk)))
            .collect::<Result<Vec<Point2D>>>()?;

        // We'll just calculate the face normals by hand here, and then we'll
        // average those to approximate the vertex normal

        let mut face_normals = Vec::new();
        let mut normal_map = HashMap::new();
        // iterate through the faces. For each face, pick three points and
        // compute the normal from those points
        let mut face_start = 0;
        for i in 0..num_faces {
            // each face has to have at least 3 points
            let p0 = vertex_pos[vertex_index[face_start]];
            let p1 = vertex_pos[vertex_index[face_start + 1]];
            let p2 = vertex_pos[vertex_index[face_start + 2]];
            let normal = (p1 - p0).cross(p2 - p0).normalize();

            // add this normal to the list of all normals for each of the given vertices
            for j in 0..face_index[i] {
                normal_map
                    .entry(vertex_index[face_start + j])
                    .or_insert(Vec::new())
                    .push(normal.clone());
            }

            face_normals.push(normal);

            face_start += face_index[i];
        }

        let vertices = (0..vertex_pos.len())
            .map(|i| {
                let v = normal_map.get(&i).expect("A vertex was never used");
                let n =
                    (v.iter().fold(Vector3::default(), |a, e| a + e) / v.len() as f64).normalize();
                Vertex::new(vertex_pos[i], n, tex_coords[i])
            })
            .collect::<Vec<Vertex>>();

        Ok(Self {
            num_faces,
            face_index,
            vertex_index,
            vertices,
            face_normals,
            bounding_box,
        })
    }
}
