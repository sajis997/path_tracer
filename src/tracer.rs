use image::{Rgb, RgbImage};
use indicatif::{ParallelProgressIterator, ProgressBar};
use rayon::prelude::*;
use rand::prelude::*;

use util::{Color,Util};
use camera::Camera;
use hit::{Hit, World};
use ray::Ray;
use std::{fs, path::Path};
use std::sync::{Arc, RwLock};


use crate::{util, camera, hit,ray};


pub struct Tracer {
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

    pub fn ray_color(&self, r: &Ray, world: &World, depth: u32) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
    
        if let Some(rec) = world.hit(r, 0.001, f32::INFINITY) {
            if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
                attenuation * self.ray_color(&scattered, world, depth - 1)
            } else {
                Color::new(0.0, 0.0, 0.0)
            }
        } else {
            let unit_direction = r.direction().normalize();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }

    pub fn trace(&self, cam: &Camera, world: &World, bar: &ProgressBar ,max_depth: u32) {
    
        // generate pixel coordinates from the image buffer
        let pixels = (0..self.image_width)
            .flat_map(|x| (0..self.image_height).map(move |y| (x, y)))
            .collect::<Vec<(u32, u32)>>();


        /////////////////////////////////////////////
        //Perform multi-sampling with parallelism
        pixels
            .par_iter()
            .progress_with(bar.clone())
            .for_each(|(x, y)| {
            
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..self.samples_per_pixel {
                let mut rng = rand::thread_rng();

                let random_u: f32 = rng.gen();
                let random_v: f32 = rng.gen();

                let u = ((*x as f32) + random_u) / ((self.image_width - 1) as f32);
                let v = 1.0 - (((*y as f32) + random_v) / ((self.image_height - 1) as f32));
                let ray = cam.get_ray(u, v); 

                pixel_color += self.ray_color(&ray, &world, max_depth);               

            }

            
            let image_buffer_lock_result = self.image_buffer.try_write();

            if !image_buffer_lock_result.is_err() {
                // conduct gamma correction over the pixel
                let mut buffer = image_buffer_lock_result.unwrap();
                buffer.put_pixel(*x, *y, Rgb(Util::gamma_correction(&pixel_color, self.samples_per_pixel)));
            }           
        });

    }

    pub fn save(&self,image_path: &str, file_name: &str){
        let folder_creation = fs::create_dir_all(image_path);

        if folder_creation.is_err() {
            panic!("Error creating the output folder");
        }

        let path = Path::new(".");
        let dirs = path.join(image_path).join(file_name);

        match self.image_buffer.try_read().unwrap().save(dirs) {
            Ok(_) => println!("Image saved successfully"),
            Err(err) => eprintln!("Error saving image: {err}",)
        };
    }
}
