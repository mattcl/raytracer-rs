use std::convert::TryFrom;
use std::fs::File;

use raytracer_rs::color::Color;
use raytracer_rs::light::{DirectionalLight, PointLight};
use raytracer_rs::material::{Checker, Material, Surface};
use raytracer_rs::math::{Matrix4, Point3D, Vector3};
use raytracer_rs::ply::Ply;
use raytracer_rs::shape::mesh::{ShadingMode, TriangleMesh};
use raytracer_rs::shape::{Plane, Transformable};
use raytracer_rs::{Camera, Scene, View};

fn main() {
    let mut scene = Scene::new();
    scene.set_view(View::new(1920, 1080));
    let cam = Camera::new(Point3D::new(0, 5, 20), &Point3D::new(0, 0, 0), 70.0);
    scene.add_camera(cam);
    scene.set_max_generations(7);

    let f = File::open("beethoven.ply").expect("could not open cube.ply");

    let ply = Ply::try_from(f).expect("could not parse ply file");

    scene.add_shape(
        Plane::new(Point3D::new(0.0, -5.0, 0.0), Vector3::J).with_material(
            Material::new(Checker::new(Color::WHITE).with_secondary(Color::BLACK))
                .with_surface(Surface::Reflective(0.3))
                .with_scale(0.1),
        ),
    );

    let mut bust = TriangleMesh::from(ply);
    let mut smooth_bust = bust.clone().with_shading(ShadingMode::Smooth);

    let transform = Matrix4([
        [1.0, 0.0, 0.0, 5.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    bust.transform(&transform).expect("could not translate");
    scene.add_shape(bust);

    let transform = Matrix4([
        [1.0, 0.0, 0.0, -5.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    smooth_bust.transform(&transform).expect("could not translate");
    scene.add_shape(smooth_bust);

    scene.add_light(DirectionalLight::default());
    scene.add_light(DirectionalLight::new(
        Vector3::new([0.2, -1.0, 0.2]).normalize(),
        Color::WHITE,
        2.0,
    ));

    scene.add_light(PointLight::new(Point3D::new(0.0, 10.0, 10.0)));
    scene.add_light(PointLight::new(Point3D::new(-7.0, 5.0, 1.0)).intensity(1500.0));
    scene.add_light(PointLight::new(Point3D::new(-2.0, 0.0, -7.0)).intensity(700.0));

    scene
        .par_raytrace()
        .iter_mut()
        .enumerate()
        .for_each(|(index, img)| img.save(format!("cam-{}.png", index)).unwrap());
}
