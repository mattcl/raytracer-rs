use std::f64::consts::PI;

use image::{imageops, DynamicImage, GenericImage};
use rayon::prelude::*;

use crate::{
    camera::Camera,
    color::Color,
    light::Light,
    material::{Surface, TextureCoord, Textured},
    math::{Point3D, Vector3},
    ray::Ray,
    shape::{Intersect, Intersection, Shape, Transformable},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct View {
    width: u32,
    height: u32,
}

impl View {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn to_plane_coord(&self, x: u32, y: u32) -> (f64, f64) {
        (
            x as f64 - self.width as f64 / 2.0,
            self.height as f64 / 2.0 - y as f64,
        )
    }
}

impl Default for View {
    fn default() -> Self {
        Self::new(800, 600)
    }
}

pub struct Scene {
    cameras: Vec<Camera>,
    lights: Vec<Light>,
    shapes: Vec<Shape>,
    view: View,
    background: Color,
    max_generations: usize,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            cameras: Vec::new(),
            lights: Vec::new(),
            shapes: Vec::new(),
            view: View::default(),
            background: Color::default(),
            max_generations: 5,
        }
    }

    pub fn set_max_generations(&mut self, max: usize) {
        self.max_generations = max
    }

    pub fn set_view(&mut self, view: View) {
        self.view = view;
    }

    pub fn add_camera(&mut self, camera: Camera) {
        self.cameras.push(camera);
    }

    pub fn add_shape<T>(&mut self, shape: T)
    where
        T: Intersect + Textured + Transformable + Into<Shape>,
    {
        self.shapes.push(shape.into());
    }

    pub fn add_light(&mut self, light: impl Into<Light>) {
        self.lights.push(light.into());
    }

    pub fn raytrace(&self) -> Vec<DynamicImage> {
        self.cameras.iter().map(|c| self.raytrace_cam(c)).collect()
    }

    pub fn par_raytrace(&self) -> Vec<DynamicImage> {
        self.cameras
            .iter()
            .map(|c| self.par_raytrace_cam(c))
            .collect()
    }

    fn raytrace_cam(&self, camera: &Camera) -> DynamicImage {
        let mut img = DynamicImage::new_rgb8(self.view.width, self.view.height);

        let d = (self.view.width as f64 / 2.0) / (camera.fov_radians() / 2.0).tan();

        for x in 0..self.view.width {
            for y in 0..self.view.height {
                let (sx, sy) = self.view.to_plane_coord(x, y);
                let v = (d * camera.forward() + sx * camera.right() + sy * camera.up()).normalize();
                let ray = Ray::new(camera.origin().clone(), v);

                let col = self.color_for(&ray);

                img.put_pixel(x, y, col.into());
            }
        }

        img
    }

    fn par_raytrace_cam(&self, camera: &Camera) -> DynamicImage {
        let mut img = DynamicImage::new_rgb8(self.view.width, self.view.height);

        let d = (self.view.width as f64 / 2.0) / (camera.fov_radians() / 2.0).tan();

        (0..self.view.width)
            .into_par_iter()
            .map(|x| {
                let mut partial = DynamicImage::new_rgb8(1, self.view.height);
                for y in 0..self.view.height {
                    let (sx, sy) = self.view.to_plane_coord(x, y);
                    let v =
                        (d * camera.forward() + sx * camera.right() + sy * camera.up()).normalize();
                    let ray = Ray::new(camera.origin().clone(), v);

                    let col = self.color_for(&ray);
                    partial.put_pixel(0, y, col.into());
                }
                (partial, x)
            })
            .collect::<Vec<(DynamicImage, u32)>>()
            .iter()
            .for_each(|(part, x)| imageops::replace(&mut img, part, *x, 0));

        img
    }

    fn diffuse(
        &self,
        shape: &Shape,
        point: &Point3D,
        normal: &Vector3,
        texture_coord: &TextureCoord,
    ) -> Color {
        let mut color = Color::BLACK;

        for light in &self.lights {
            let dir = light.direction_from(point);
            let shadow = Ray::new(point + normal * 1e-10_f64, dir.clone());

            let intensity = match self.get_closest_intersection(&shadow) {
                Some(Intersection { distance: d, .. }) if light.distance(&point) > d => 0.0,
                _ => light.intensity_at(&point),
            };

            let power = normal.dot(dir).max(0.0) * intensity;
            let reflected = shape.material().albedo / PI;
            color =
                color + shape.material().color(&texture_coord) * light.color() * power * reflected;
        }

        color
    }

    fn fresnel(&self, ray: &Ray, normal: &Vector3, refractive_index: f64) -> f64 {
        let i_dot_n = ray.direction().dot(normal);
        let mut eta_i = 1.0;
        let mut eta_t = refractive_index;

        if i_dot_n > 0.0 {
            eta_i = eta_t;
            eta_t = 1.0;
        }

        let st = eta_i / eta_t * (1.0 - i_dot_n * i_dot_n).max(0.0).sqrt();
        if st > 1.0 {
            return 1.0;
        }

        let ct = (1.0 - st * st).max(0.0).sqrt();
        let ci = ct.abs();
        let s = ((eta_t * ci) - (eta_i * ct)) / ((eta_t * ci) + (eta_i * ct));
        let p = ((eta_i * ci) - (eta_t * ct)) / ((eta_i * ci) + (eta_t * ct));

        (s * s + p * p) / 2.0
    }

    fn color_at(&self, ray: &Ray, intersection: &Intersection) -> Option<Color> {
        if ray.generation() >= self.max_generations {
            return Some(Color::BLACK);
        }

        let point = match intersection.location {
            Some(point) => point,
            None => ray.point_at(intersection.distance),
        };

        let hit_normal = match intersection.normal {
            Some(normal) => normal,
            None => intersection.obj.normal_at(&point)?,
        };

        let texture_coord = match intersection.tex_coord {
            Some(coord) => coord,
            None => intersection.obj.texture_coord(&point),
        };

        let color = match intersection.obj.material().surface {
            Surface::Diffuse => self.diffuse(intersection.obj, &point, &hit_normal, &texture_coord),
            Surface::Reflective(reflectivity) => {
                let mut color = self.diffuse(intersection.obj, &point, &hit_normal, &texture_coord);
                let reflection = ray.reflect(&hit_normal, &point, 1e-10_f64);
                color = color * (1.0 - reflectivity);
                color + self.color_for(&reflection) * reflectivity
            }
            Surface::Refractive {
                index,
                transparency,
            } => {
                let mut refract_color = Color::BLACK;
                let kr = self.fresnel(ray, &hit_normal, index);
                let surface_color = intersection.obj.material().color(&texture_coord);

                if kr < 1.0 {
                    if let Some(transmission) = ray.refract(&hit_normal, &point, 1e-10_f64, index) {
                        refract_color = self.color_for(&transmission);
                    }
                }

                let reflection = ray.reflect(&hit_normal, &point, 1e-10_f64);
                let reflect_color = self.color_for(&reflection);
                let color = (1.0 - kr) * refract_color + kr * reflect_color;

                color * transparency * surface_color
            }
        };

        Some(color)
    }

    fn color_for(&self, ray: &Ray) -> Color {
        match self.get_closest_intersection(ray) {
            Some(intersection) => self.color_at(ray, &intersection).unwrap_or(Color::BLACK),
            None => self.background.clone(),
        }
    }

    fn get_closest_intersection(&self, ray: &Ray) -> Option<Intersection> {
        self.shapes
            .iter()
            .filter_map(|s| s.intersect(ray))
            .min_by(|a, b| a.partial_cmp(&b).unwrap())
    }
}
