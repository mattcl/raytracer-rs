use criterion::{criterion_group, BenchmarkId, Criterion};

use raytracer_rs::color::Color;
use raytracer_rs::light::{DirectionalLight, PointLight};
use raytracer_rs::material::{Checker, Material, Surface};
use raytracer_rs::math::{Point3D, Vector3};
use raytracer_rs::shape::{Plane, Sphere};
use raytracer_rs::{Camera, Scene, View};

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Raytracing");

    let mut scene = Scene::new();
    scene.set_view(View::new(1920, 1080));
    let cam = Camera::new(
        Point3D::new(0.0, 20.0, -20.0),
        &Point3D::new(0.0, 0.0, 2.5),
        70.0,
    );
    scene.add_camera(cam);

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

    scene.add_shape(
        Plane::new(Point3D::new(0.0, -10.0, 0.0), Vector3::J).with_material(
            Material::new(Checker::new(Color::WHITE).with_secondary(Color::BLACK))
                .with_surface(Surface::Reflective(0.3))
                .with_scale(0.1),
        ),
    );

    scene.add_light(DirectionalLight::default());
    scene.add_light(DirectionalLight::new(
        Vector3::new([0.2, -1.0, 0.2]).normalize(),
        Color::WHITE,
        2.0,
    ));

    scene.add_light(PointLight::new(Point3D::new(10.0, 10.0, 1.0)));
    scene.add_light(PointLight::new(Point3D::new(-7.0, 5.0, 1.0)).intensity(1500.0));
    scene.add_light(PointLight::new(Point3D::new(-2.0, 0.0, -7.0)).intensity(700.0));

    for width in [600, 800, 1000, 1200, 1500].iter() {
        group.bench_with_input(
            BenchmarkId::new("single-threaded", width),
            width,
            |b, width| {
                scene.set_view(View::new(*width, *width));
                b.iter(|| scene.raytrace().len())
            },
        );

        group.bench_with_input(BenchmarkId::new("parallel", width), width, |b, width| {
            scene.set_view(View::new(*width, *width));
            b.iter(|| scene.par_raytrace().len())
        });

        group.bench_with_input(BenchmarkId::new("parallel-unchunked", width), width, |b, width| {
            scene.set_view(View::new(*width, *width));
            b.iter(|| scene.par_raytrace_unchunked().len())
        });
    }

    group.finish();
}

criterion_group!(benches, bench);
