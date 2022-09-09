//main.rs - is the crate root file

mod camera;
mod hit;
mod ray;
mod sphere;
mod vec; // to use Vec3 in the program, add a reference with mod keyword

use camera::Camera;
use hit::{Hit, World};
use image::{ImageBuffer, Rgb, RgbImage};
use rand::prelude::*;
use ray::Ray;
use sphere::Sphere;
use vec::{Color, Point3, Vec3};

fn ray_color(r: &Ray, world: &World, depth: u32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        let target = rec.p + Vec3::random_in_hemisphere(rec.normal);
        let r = Ray::new(rec.p, target - rec.p);
        0.5 * ray_color(&r, world, depth - 1)
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
    const SAMPLES_PER_PIXEL: u32 = 50;
    const MAX_DEPTH: u32 = 15;

    //image plane
    let mut buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);

    //world
    let mut world = World::new(); // crate an empty world

    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    //camera setup
    let cam = Camera::new();
    let mut rng = rand::thread_rng();

    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);

        for _ in 0..SAMPLES_PER_PIXEL {
            let random_u: f64 = rng.gen();
            let random_v: f64 = rng.gen();

            let u = ((x as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
            let v = 1.0 - (((y as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64));
            let ray = cam.get_ray(u, v);

            pixel_color += ray_color(&ray, &world, MAX_DEPTH);
        }

        *pixel = Rgb(pixel_color.gamma_correction(SAMPLES_PER_PIXEL));
    }

    match buffer.save("gamma-correction.png") {
        Err(e) => panic!("Error writing file {}", e),
        Ok(()) => println!("Saving Done!"),
    }
}
