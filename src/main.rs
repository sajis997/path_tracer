//main.rs - is the crate root file

mod camera;
mod hit;
mod material;
mod ray;
mod sphere;
mod vec; // to use Vec3 in the program, add a reference with mod keyword

use std::rc::Rc;

use crate::camera::Camera;
use crate::hit::{Hit, World};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec::{Color, Point3, Vec3};
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressFinish, ProgressIterator, ProgressStyle};
use rand::prelude::*;

fn ray_color(r: &Ray, world: &World, depth: u32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    //image setup
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 800;
    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 15;

    //image plane
    let mut buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);

    //world
    let mut world = World::new(); // crate an empty world

    let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let mat_left = Rc::new(Dielectric::new(1.5));
    let mat_left_inner = Rc::new(Dielectric::new(1.5));
    let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    let sphere_ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, mat_ground);
    let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, mat_center);
    let sphere_left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, mat_left);
    let sphere_left_inner = Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.4, mat_left_inner);
    let sphere_right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right);

    world.push(Box::new(sphere_ground));
    world.push(Box::new(sphere_center));
    world.push(Box::new(sphere_left));
    world.push(Box::new(sphere_left_inner));
    world.push(Box::new(sphere_right));

    //camera setup
    let cam = Camera::new();
    let mut rng = rand::thread_rng();

    println!("Rendering Scene ...");

    let bar = ProgressBar::new((IMAGE_WIDTH * IMAGE_HEIGHT) as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{wide_bar:.green/white}] {percent}% - {elapsed_precise} elapsed {msg}",
            )
            .progress_chars("#>-")
            .on_finish(ProgressFinish::WithMessage("-- Done!".into())),
    );

    for (x, y, pixel) in buffer.enumerate_pixels_mut().progress_with(bar) {
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);

        for _ in 0..SAMPLES_PER_PIXEL {
            let random_u: f64 = rng.gen();
            let random_v: f64 = rng.gen();

            let u = ((x as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
            let v = 1.0 - (((y as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64));
            let ray = cam.get_ray(u, v);

            pixel_color += ray_color(&ray, &world, MAX_DEPTH);
        }

        //gamma correct the pixel color
        *pixel = Rgb(pixel_color.gamma_correction(SAMPLES_PER_PIXEL));
    }

    match buffer.save("schlick-approximation.png") {
        Err(e) => panic!("Error writing file {}", e),
        Ok(()) => println!("Saving Done!"),
    }
}
