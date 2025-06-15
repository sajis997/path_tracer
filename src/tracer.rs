use image::{Rgb, RgbImage};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressFinish, ProgressStyle};
use rayon::prelude::*;
use rand::prelude::*;

use crate::utils::util::{Color,Util};
use crate::camera::Camera;
use crate::hit::{Hit, World};
use crate::ray::Ray;
use std::{fs, path::Path};
use std::sync::RwLock;

pub struct Tracer {

    // the fields within the struct are private
    image_buffer: RwLock<RgbImage>,
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
}

impl Tracer {
    pub fn new(width: u32,
            height: u32,
            samples: u32) -> Self {

        Tracer {
            image_buffer: RwLock::new(RgbImage::new(width, height)),
            image_width: width,
            image_height: height,
            samples_per_pixel: samples,
        }
    }

    fn ray_color(&self, r: &Ray, world: &World, depth: u32) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        match world.hit(r,0.001, f32::INFINITY) {
            Some(rec) => {
                match rec.mat.scatter(r,&rec) {
                    Some((attenuation, scattered)) => attenuation * self.ray_color(&scattered, world, depth - 1),
                    None => Color::new(0.0, 0.0, 0.0),
                }
            },
            None => {
                let unit_direction = r.direction().normalize();
                let t = 0.5 * (unit_direction.y + 1.0);
                (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
            }
        }
    }

    pub fn trace(&self, cam: &Camera, world: &World, max_depth: u32) {

        let style = ProgressStyle::default_bar().template(
            "{spinner:.green} [{wide_bar:.green/white}] {percent}% - {elapsed_precise} elapsed {msg}",
        );
        let progress_bar = ProgressBar::new((self.image_width * self.image_height) as u64);
        progress_bar.set_style(style.unwrap().progress_chars("#>-"));

        match self.image_buffer.write() {
            Ok(mut locked_buffer) => {
                locked_buffer
                    .par_enumerate_pixels_mut()
                    .progress_with(progress_bar.clone())
                    .for_each(|( x, y,px_out)| {

                        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                        let mut rng = rand::thread_rng();

                        // generate random samples
                        let random_samples : Vec<(f32, f32)> = (0..self.samples_per_pixel)
                            .map(|_| (rng.gen(), rng.gen()))
                            .collect();

                        for (random_U,random_V) in random_samples {

                            let u = ((x as f32) + random_U) / ((self.image_width - 1) as f32);
                            let v = 1.0 - (((y as f32) + random_V) / ((self.image_height - 1) as f32));
                            let ray = cam.get_ray(u, v); 
            
                            pixel_color += self.ray_color(&ray, &world, max_depth);                       
                        }
                        *px_out = Rgb(Util::gamma_correction(&pixel_color, self.samples_per_pixel));
                    });
            },
            Err(_) => panic!("Error locking the image buffer"),
        }
        progress_bar.with_finish(ProgressFinish::WithMessage("\nScene Rendering Completed.".into()));
    }

    pub fn save(&self,image_path: &str, file_name: &str){
        let folder_creation = fs::create_dir_all(image_path);

        if folder_creation.is_err() {
            panic!("Error creating the output folder");
        }

        let path = Path::new(".");
        let dirs = path.join(image_path).join(file_name);

        match self.image_buffer.read()
        {
            Ok(locked_buffer) => match locked_buffer.save(dirs) {
                Ok(_) => println!("Image saved successfully"),
                Err(err) => eprintln!("Error saving image: {err}",)
            },
            Err(_) => panic!("Error locking the image buffer"),
        };
    }
}
