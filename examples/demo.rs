use image::io::Reader as ImageReader;
use image::DynamicImage;
use raytracer_rs::color::Color;
use raytracer_rs::light::{DirectionalLight, PointLight};
use raytracer_rs::material::{Checker, Material, Surface};
use raytracer_rs::math::{Point3D, Vector3};
use raytracer_rs::shape::{Plane, Sphere, Triangle};
use raytracer_rs::{Camera, Scene, View};

fn main() {
    let mut scene = Scene::new();
    scene.set_view(View::new(1920, 1080));
    // scene.add_camera(Camera::default());

    let cam2 = Camera::new(Point3D::new(0, 20, -20), &Point3D::new(0, 0, 2.5), 70.0);
    scene.add_camera(cam2);
    let cam3 = Camera::new(Point3D::new(20, 20, 60), &Point3D::new(0, 0, 0), 70.0);
    scene.add_camera(cam3);

    // let cam2 = Camera::new(Point3D::new(0, 0, 40), &Point3D::new(0, 0, 0), 70.0);
    // scene.add_camera(cam2);

    scene.set_max_generations(7);

    scene.add_shape(
        Sphere::new(Point3D::new(-5.0, 0.0, 8.0), 5.0)
            .with_material(Material::new(Color::BLUE).with_surface(Surface::Reflective(1.0))),
    );

    scene.add_shape(
        Sphere::new(Point3D::new(5.0, 0.0, 0.0), 6.0)
            .with_material(Material::new(Checker::default()).with_scale(5.0)),
    );

    scene.add_shape(
        Sphere::new(Point3D::new(-5.0, 0.0, 1.0), 2.0)
            .with_material(Material::new(Checker::new(Color::RED)).with_scale(2.0)),
    );

    scene.add_shape(
        Sphere::new(Point3D::new(0.0, 35.0, -10.0), 15.0).with_material(Material::new(Color::BLUE)),
    );

    let triangle = Triangle::new(
        Point3D::new(-16, -1, 17),
        Point3D::new(-14, -1, 19),
        Point3D::new(-12, -1, 17),
    )
    .with_material(Material::new(Color::RED));

    scene.add_shape(triangle);

    let fine: DynamicImage = ImageReader::open("this-is-fine.jpg")
        .unwrap()
        .decode()
        .unwrap();
    scene.add_shape(
        Sphere::new(Point3D::new(-8.0, -2.0, -1.0), 2.0)
            .with_material(Material::new(fine).with_scale(2.0)),
    );

    scene.add_shape(
        Plane::new(Point3D::new(0.0, -10.0, 0.0), Vector3::J).with_material(
            Material::new(Checker::new(Color::WHITE).with_secondary(Color::BLACK))
                .with_surface(Surface::Reflective(0.3))
                .with_scale(0.1),
        ),
    );

    // let mesh_raw = fs::read_to_string("test.geo").expect("Could not open test.geo");
    // let mesh_raw = fs::read_to_string("cow.geo").expect("Could not open test.geo");
    // let geo = GeoMesh::try_from(mesh_raw.as_str()).expect("Could not parse test.geo");
    // let tri_mesh = TriangleMesh::from(geo);

    // scene.add_shape(
    //     tri_mesh
    // );

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
