use ply_rs::{
    self,
    parser::Parser,
    ply::{Property, PropertyAccess},
};
use std::{
    collections::HashMap,
    convert::TryFrom,
    f64::{INFINITY, NEG_INFINITY},
    fs::File,
};

use crate::{
    error::{RTError, Result},
    math::{Point2D, Point3D, Vector3},
    shape::{mesh::Vertex, BoundingBox},
};

impl PropertyAccess for Point3D {
    fn new() -> Self {
        Self::default()
    }

    fn set_property(&mut self, name: String, property: Property) {
        match (name.as_ref(), property) {
            ("x", Property::Float(val)) => self[0] = val.into(),
            ("y", Property::Float(val)) => self[1] = val.into(),
            ("z", Property::Float(val)) => self[2] = val.into(),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct PlyFace {
    vertex_index: Vec<usize>,
}

impl PropertyAccess for PlyFace {
    fn new() -> Self {
        Self::default()
    }

    fn set_property(&mut self, name: String, property: Property) {
        match (name.as_ref(), property) {
            ("vertex_index", Property::ListInt(val))
            | ("vertex_indices", Property::ListInt(val)) => {
                self.vertex_index = val.into_iter().map(|v| v as usize).collect()
            }
            _ => unreachable!(),
        }

        // winding?
        // self.vertex_index.reverse()
    }
}

#[derive(Default, Debug, Clone)]
pub struct Ply {
    pub faces: Vec<Vec<usize>>,
    pub vertices: Vec<Vertex>,
    pub face_normals: Vec<Vector3>,
    pub bounding_box: BoundingBox,
}

impl TryFrom<File> for Ply {
    type Error = RTError;

    fn try_from(file: File) -> Result<Self> {
        let mut f = std::io::BufReader::new(file);

        let vertex_parser = Parser::<Point3D>::new();
        let face_parser = Parser::<PlyFace>::new();

        let header = vertex_parser.read_header(&mut f)?;

        let mut vertex_raw = Vec::new();
        let mut face_raw = Vec::new();
        for (_, e) in &header.elements {
            match e.name.as_ref() {
                "vertex" => {
                    vertex_raw = vertex_parser.read_payload_for_element(&mut f, &e, &header)?
                }
                "face" => face_raw = face_parser.read_payload_for_element(&mut f, &e, &header)?,
                _ => unreachable!(),
            }
        }

        let mut min = [INFINITY; 3];
        let mut max = [NEG_INFINITY; 3];
        for point in &vertex_raw {
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

        let mut face_normals = Vec::new();
        let mut normal_map: HashMap<usize, Vec<Vector3>> = HashMap::new();
        // iterate through the faces. For each face, pick three points and
        // compute the normal from those points
        for i in 0..face_raw.len() {
            let face = &face_raw[i];
            let p0 = vertex_raw[face.vertex_index[0]];
            let p1 = vertex_raw[face.vertex_index[1]];
            let p2 = vertex_raw[face.vertex_index[2]];
            let normal = (p1 - p0).cross(p2 - p0).normalize();

            for j in 0..face.vertex_index.len() {
                normal_map
                    .entry(face.vertex_index[j])
                    .or_default()
                    .push(normal);
            }

            face_normals.push(normal);
        }

        let vertices = (0..vertex_raw.len())
            .map(|i| {
                let v = normal_map.get(&i).expect("A vertex was never used");
                let n =
                    (v.iter().fold(Vector3::default(), |a, e| a + e) / v.len() as f64).normalize();
                Vertex::new(vertex_raw[i], n, Point2D::default())
            })
            .collect::<Vec<Vertex>>();

        let faces = face_raw.into_iter().map(|v| v.vertex_index).collect();

        Ok(Self {
            faces,
            vertices,
            face_normals,
            bounding_box,
        })
    }
}
