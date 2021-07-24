use std::convert::TryFrom;
use std::fs;

use raytracer_rs::color::Color;
use raytracer_rs::light::{DirectionalLight, PointLight};
use raytracer_rs::material::{Checker, Material, Surface};
use raytracer_rs::math::{Point3D, Vector3};
use raytracer_rs::shape::mesh::TriangleMesh;
use raytracer_rs::shape::Plane;
use raytracer_rs::util::GeoMesh;
use raytracer_rs::{Camera, Scene, View};

fn main() {
    let mut scene = Scene::new();
    scene.set_view(View::new(800, 600));
    let cam = Camera::new(Point3D::new(20, 20, 50), &Point3D::new(0, 0, 0), 70.0);
    scene.add_camera(cam);
    scene.set_max_generations(7);

    // let mesh_raw = fs::read_to_string("test.geo").expect("Could not open test.geo");
    let mesh_raw = fs::read_to_string("cow.geo").expect("Could not open test.geo");
    let geo = GeoMesh::try_from(mesh_raw.as_str()).expect("Could not parse test.geo");
    scene.add_shape(
        Plane::new(Point3D::new(0.0, -5.0, 0.0), Vector3::J).with_material(
            Material::new(Checker::new(Color::WHITE).with_secondary(Color::BLACK))
                .with_surface(Surface::Reflective(0.3))
                .with_scale(0.1),
        ),
    );

    let tri_mesh = TriangleMesh::from(geo);

    scene.add_shape(tri_mesh);

    scene.add_light(DirectionalLight::default());
    scene.add_light(DirectionalLight::new(
        Vector3::new([0.2, -1.0, 0.2]).normalize(),
        Color::WHITE,
        2.0,
    ));

    scene.add_light(PointLight::new(Point3D::new(10.0, 10.0, 1.0)));
    scene.add_light(PointLight::new(Point3D::new(-7.0, 5.0, 1.0)).intensity(1500.0));
    scene.add_light(PointLight::new(Point3D::new(-2.0, 0.0, -7.0)).intensity(700.0));

    scene
        .par_raytrace()
        .iter_mut()
        .enumerate()
        .for_each(|(index, img)| img.save(format!("cam-{}.png", index)).unwrap());
}
