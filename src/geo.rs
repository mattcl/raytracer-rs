// so this whole thing is ridiculously complicated, but I wanted to play with
// the nom crate, so there's that
use std::{
    collections::HashMap,
    f64::{INFINITY, NEG_INFINITY},
};

use nom::{
    character::complete::{multispace0, newline, space0},
    combinator::map,
    multi::many1,
    number::complete::double,
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::{
    error::{RTError, Result},
    math::{Point2D, Point3D, Vector3},
    shape::{mesh::Vertex, BoundingBox},
};

fn usize_parser(i: &str) -> IResult<&str, usize> {
    map(preceded(space0, double), |d| d as usize)(i)
}

fn int_list(i: &str) -> IResult<&str, Vec<usize>> {
    many1(usize_parser)(i)
}

fn vertex(i: &str) -> IResult<&str, Point3D> {
    let (input, (x, y, z)) = tuple((
        preceded(space0, double),
        preceded(space0, double),
        preceded(space0, double),
    ))(i)?;
    Ok((input, Point3D::new(x, y, z)))
}

fn vertices(i: &str) -> IResult<&str, Vec<Point3D>> {
    many1(vertex)(i)
}

fn tex_coord(i: &str) -> IResult<&str, Point2D> {
    let (input, (u, v)) = tuple((preceded(space0, double), preceded(space0, double)))(i)?;
    Ok((input, Point2D::new(u, v)))
}

fn tex_coords(i: &str) -> IResult<&str, Vec<Point2D>> {
    many1(tex_coord)(i)
}

fn geo_parser(i: &str) -> IResult<&str, Geo> {
    let (i, num_faces) = delimited(multispace0, usize_parser, tuple((space0, newline)))(i)?;
    let (i, faces) = delimited(multispace0, int_list, tuple((space0, newline)))(i)?;
    let (i, vertex_index) = delimited(multispace0, int_list, tuple((space0, newline)))(i)?;
    let (i, verts) = delimited(multispace0, vertices, tuple((space0, newline)))(i)?;
    // So I'm convinced the scratchapixel test.geo has the wrong values
    // for the normals so we're just going to ignore this and derive
    // them ourselves
    let (i, _normals) = delimited(multispace0, vertices, tuple((space0, newline)))(i)?;
    let (i, tex) = delimited(multispace0, tex_coords, tuple((space0, newline)))(i)?;
    Ok((
        i,
        Geo {
            num_faces,
            faces,
            vertex_index,
            vertices: verts,
            tex_coords: tex,
        },
    ))
}

pub struct Geo {
    num_faces: usize,
    faces: Vec<usize>,
    vertex_index: Vec<usize>,
    vertices: Vec<Point3D>,
    tex_coords: Vec<Point2D>,
}

impl Geo {
    pub fn from_str(s: &str) -> Result<Self> {
        geo_parser(s).and_then(|(_, g)| Ok(g)).or_else(|e| {
            Err(RTError::Error(format!(
                "Unable to parse geo: {}",
                e.to_string()
            )))
        })
    }
}

#[derive(Debug, Clone)]
pub struct GeoMesh {
    pub num_faces: usize,
    pub face_index: Vec<usize>,
    pub vertex_index: Vec<usize>,
    pub vertices: Vec<Vertex>,
    pub face_normals: Vec<Vector3>,
    pub bounding_box: BoundingBox,
}

impl GeoMesh {
    pub fn from_str(s: &str) -> Result<Self> {
        let geo = Geo::from_str(s)?;

        let mut min = [INFINITY; 3];
        let mut max = [NEG_INFINITY; 3];
        for point in &geo.vertices {
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

        // We'll just calculate the face normals by hand here, and then we'll
        // average those to approximate the vertex normal

        let mut face_normals = Vec::new();
        let mut normal_map = HashMap::new();
        // iterate through the faces. For each face, pick three points and
        // compute the normal from those points
        let mut face_start = 0;
        for i in 0..geo.num_faces {
            // each face has to have at least 3 points
            let p0 = geo.vertices[geo.vertex_index[face_start]];
            let p1 = geo.vertices[geo.vertex_index[face_start + 1]];
            let p2 = geo.vertices[geo.vertex_index[face_start + 2]];
            let normal = (p1 - p0).cross(p2 - p0).normalize();

            // add this normal to the list of all normals for each of the given vertices
            for j in 0..geo.faces[i] {
                normal_map
                    .entry(geo.vertex_index[face_start + j])
                    .or_insert(Vec::new())
                    .push(normal.clone());
            }

            face_normals.push(normal);

            face_start += geo.faces[i];
        }

        let vertices = (0..geo.vertices.len())
            .map(|i| {
                let v = normal_map.get(&i).expect("A vertex was never used");
                let n =
                    (v.iter().fold(Vector3::default(), |a, e| a + e) / v.len() as f64).normalize();
                Vertex::new(geo.vertices[i], n, geo.tex_coords[i])
            })
            .collect::<Vec<Vertex>>();

        Ok(Self {
            num_faces: geo.num_faces,
            face_index: geo.faces,
            vertex_index: geo.vertex_index,
            vertices,
            face_normals,
            bounding_box,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_list_parsing() {
        assert_eq!(int_list("2 3 4 5"), Ok(("", vec![2, 3, 4, 5])));
    }

    #[test]
    fn vertex_parsing() {
        assert_eq!(vertex("1 1 1 2 2"), Ok((" 2 2", Point3D::new(1, 1, 1))));
        assert_eq!(vertex(" -1 2.123 2"), Ok(("", Point3D::new(-1, 2.123, 2))));
        assert!(vertex("1 1").is_err());
    }

    #[test]
    fn vertices_parsing() {
        let r = vertices("1 1 1 2 2 2 3 3 3\n4 4 4");
        assert_eq!(
            r,
            Ok((
                "\n4 4 4",
                vec![
                    Point3D::new(1, 1, 1),
                    Point3D::new(2, 2, 2),
                    Point3D::new(3, 3, 3)
                ]
            ))
        );
    }
}
