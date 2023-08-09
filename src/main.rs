//main.rs - is the crate root file

// declare the following modules. The compiler will look for the module's code
// in the following places:
// 1. src/*

mod aabb;
mod axis;
mod camera;
mod hit;
mod material;
mod ray;
mod sphere;
mod util;

// the following use keywords will bring the paths into the scope
use camera::Camera;
use hit::{Hit, World};
use material::{Dielectric, Lambertian, Metal};
use ray::Ray;
use sphere::Sphere;

use glam::Vec3;
use image::Rgb;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressFinish, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use util::{Color, Point3, Util};

fn ray_color(r: &Ray, world: &World, depth: u32) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f32::INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = r.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn random_scene() -> World {
    let mut rng = rand::thread_rng();
    let mut world = World::with_capacity(550);

    let ground_mat = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat);

    world.push(Box::new(ground_sphere));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(
                (a as f32) + rng.gen_range(0.0..0.9),
                0.2,
                (b as f32) + rng.gen_range(0.0..0.9),
            );

            if choose_mat < 0.8 {
                // Diffuse
                let albedo = Util::random(0.0..1.0) * Util::random(0.0..1.0);
                let sphere_mat = Arc::new(Lambertian::new(albedo));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            } else if choose_mat < 0.95 {
                // Metal
                let albedo = Util::random(0.4..1.0);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_mat = Arc::new(Metal::new(albedo, fuzz));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            } else {
                // Glass
                let sphere_mat = Arc::new(Dielectric::new(1.5));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);
    let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

    world.push(Box::new(sphere1));
    world.push(Box::new(sphere2));
    world.push(Box::new(sphere3));

    world
}

//image setup
const ASPECT_RATIO: f32 = 3.0 / 2.0;
const IMAGE_WIDTH: u32 = 800;
const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f32) / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 500;
const MAX_DEPTH: u32 = 50;
const CHANNELS: u32 = 3;
const IMAGE_OUT_DIR: &str = "output";
const IMAGE_FILE_NAME: &str = "parallel-rendering-bigger-4.png";

fn main() {
    let folder_creation = fs::create_dir_all(IMAGE_OUT_DIR);

    if folder_creation.is_err() {
        panic!("Error creating the output folder");
    }

    let path = Path::new(".");
    let dirs = path.join(IMAGE_OUT_DIR).join(IMAGE_FILE_NAME);

    //world
    let world = random_scene(); // crate an empty world

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    println!("Rendering Scene ...");

    //image plane
    let mut buffer = vec![0u8; (IMAGE_WIDTH * IMAGE_HEIGHT * CHANNELS) as usize];

    let bands: Vec<(usize, &mut [u8])> = buffer
        .chunks_mut((IMAGE_WIDTH * CHANNELS) as usize)
        .enumerate()
        .collect();

    let style = ProgressStyle::default_bar().template(
        "{spinner:.green} [{wide_bar:.green/white}] {percent}% - {elapsed_precise} elapsed {msg}",
    );
    let bar = ProgressBar::new(IMAGE_HEIGHT as u64);
    bar.set_style(style.unwrap().progress_chars("#>-"));

    /*
        1. converts the collection into parallel iterator - each band within the bands is assigned to the iterator that executes in parallel
        2. for each band we loop though the pixels and accumulate pixel color with multi-sampling
    */
    bands
        .into_par_iter()
        .progress_with(bar.with_finish(ProgressFinish::WithMessage("-- Done!".into())))
        .for_each(|(i, band)| {
            // get the image band - in other words the scanline
            // go through all the pixels within the scanline
            for x in 0..IMAGE_WIDTH {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..SAMPLES_PER_PIXEL {
                    let mut rng = rand::thread_rng();
                    let random_u: f32 = rng.gen();
                    let random_v: f32 = rng.gen();

                    let u = ((x as f32) + random_u) / ((IMAGE_WIDTH - 1) as f32);
                    let v = 1.0 - (((i as f32) + random_v) / ((IMAGE_HEIGHT - 1) as f32));
                    let ray = cam.get_ray(u, v);

                    pixel_color += ray_color(&ray, &world, MAX_DEPTH);
                }

                // conduct gamma correction over the pixel
                let pixel = Rgb(Util::gamma_correction(&pixel_color, SAMPLES_PER_PIXEL));

                band[(x * CHANNELS) as usize] = pixel[0];
                band[(x * CHANNELS + 1) as usize] = pixel[1];
                band[(x * CHANNELS + 2) as usize] = pixel[2];
            }
        });

    //save the raw data
    match image::save_buffer(
        dirs,
        &buffer,
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        image::ColorType::Rgb8,
    ) {
        Err(e) => panic!("Error writing file: {} and the error is: {}",IMAGE_FILE_NAME, e),
        Ok(()) => println!("Saving of Rendered Image is Done at: {}",IMAGE_FILE_NAME),
    };
}
