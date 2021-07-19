use image::DynamicImage;
use raytracer_rs::color::Color;
use raytracer_rs::light::{DirectionalLight, PointLight};
use raytracer_rs::material::{Checker, Material, Surface};
use raytracer_rs::math::{Point3D, Vector3};
use raytracer_rs::shape::{Plane, Sphere};
use raytracer_rs::{Camera, Scene, View};
use image::io::Reader as ImageReader;

fn main() {
    let mut scene = Scene::new();
    scene.set_view(View::new(1920, 1080));
    scene.add_camera(Camera::default());

    let cam2 = Camera::new(
        Point3D::new(0.0, 20.0, -20.0),
        &Point3D::new(0.0, 0.0, 2.5),
        70.0,
    );
    scene.add_camera(cam2);

    scene.set_max_generations(7);

    scene.add_shape(Sphere::with_material(
        Point3D::new(-5.0, 0.0, 8.0),
        5.0,
        Material::new(Color::BLUE).surface(Surface::Reflective(1.0))
    ));

    scene.add_shape(Sphere::with_material(
        Point3D::new(5.0, 0.0, 0.0),
        6.0,
        Material::new(Checker::default()).scale(4.0),
    ));

    scene.add_shape(Sphere::with_material(
        Point3D::new(-5.0, 0.0, 1.0),
        2.0,
        Material::new(Checker::new(Color::RED)).scale(2.0),
    ));

    scene.add_shape(Sphere::with_material(
        Point3D::new(0.0, 35.0, -10.0),
        15.0,
        Material::new(Color::BLUE),
    ));

    let fine: DynamicImage = ImageReader::open("this-is-fine.jpg").unwrap().decode().unwrap();
    scene.add_shape(Sphere::with_material(
        Point3D::new(-8.0, -2.0, -1.0),
        2.0,
        Material::new(fine).scale(2.0),
    ));

    scene.add_shape(Plane::with_material(
        Point3D::new(0.0, -10.0, 0.0),
        Vector3::J,
        Material::new(Checker::new(Color::WHITE).secondary(Color::BLACK)).surface(Surface::Reflective(0.3)).scale(0.1),
    ));

    scene.add_light(DirectionalLight::default());
    scene.add_light(DirectionalLight::new(
        Vector3::new(0.2, -1.0, 0.2).normalize(),
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
