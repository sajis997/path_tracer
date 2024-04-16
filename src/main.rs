//main.rs - is the crate root file of a binary crate with the same name as the package.

// declare the following modules. The compiler will look for the module's code
// in the following places:
// 1. src/*


mod camera;
mod hit;
mod material;
mod ray;
mod utils;
mod tracer;
mod primitives;
mod accelerators;

// the following use keywords will bring the paths into the scope
use camera::Camera;
use hit::World;
use material::{Dielectric, Lambertian, Metal};
use primitives::sphere::Sphere;
use accelerators::accelerator::Accelerator;
use accelerators::bvh;


use glam::Vec3;
use indicatif::{ProgressBar, ProgressFinish, ProgressStyle};
use rand::prelude::*;
use utils::util::Color;
use utils::util::Point3;
use utils::util::Util;

use tracer::Tracer;

fn random_scene() -> World {
    let mut rng = rand::thread_rng();
    let mut world = World::with_capacity(550);

    let ground_mat = Lambertian::new(Color::new(0.5, 0.5, 0.5));
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
                let sphere_mat = Lambertian::new(albedo);
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            } else if choose_mat < 0.95 {
                // Metal
                let albedo = Util::random(0.4..1.0);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_mat = Metal::new(albedo, fuzz);
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            } else {
                // Glass
                let sphere_mat = Dielectric::new(1.5);
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            }
        }
    }

    let mat1 = Dielectric::new(1.5);
    let mat2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let mat3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);

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
const IMAGE_WIDTH: u32 = 1024;
const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f32) / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 1000;
const MAX_DEPTH: u32 = 100;
const IMAGE_OUT_DIR: &str = "output";
const IMAGE_FILE_NAME: &str = "parallel-pixel-rendering.png";

fn main() {
    //world
    let world = random_scene(); // crate an empty world

    // create a new bvh instance
    let mut bvh = bvh::Bvh::new(Some(4),Some(bvh::SplitMethod::Middle));

    // build the bvh
    bvh.build(&world);

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

    let tracer = Tracer::new(IMAGE_WIDTH,IMAGE_HEIGHT,SAMPLES_PER_PIXEL);

    let style = ProgressStyle::default_bar().template(
        "{spinner:.green} [{wide_bar:.green/white}] {percent}% - {elapsed_precise} elapsed {msg}",
    );  
    let progress_bar = ProgressBar::new((IMAGE_WIDTH * IMAGE_HEIGHT) as u64);
    progress_bar.set_style(style.unwrap().progress_chars("#>-"));
    

    println!("Rendering Scene ...");
    tracer.trace(&cam, &world, &progress_bar ,MAX_DEPTH);

    progress_bar.with_finish(ProgressFinish::WithMessage("\nScene Rendering Completed.".into()));       

    tracer.save(IMAGE_OUT_DIR, IMAGE_FILE_NAME);
}
