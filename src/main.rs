

mod vec; // to use Vec3 in the program, add a reference with mod keyword
mod ray;
mod hit;
mod sphere;
mod camera;


use rand::prelude::*;
use image::{RgbImage,ImageBuffer,Rgb};
use vec::{Vec3,Color,Point3};
use ray::Ray;
use sphere::Sphere;
use hit::{Hit,World};
use camera::Camera;

fn ray_color(r: &Ray, world: &World) -> Color {
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0))
    } else {
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    
    //image setup
    const ASPECT_RATIO: f64 = 16.0/ 9.0;
    const IMAGE_WIDTH: u32 = 800;
    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 500;
    
    //image plane
    let mut buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH as u32,IMAGE_HEIGHT as u32);

    //world
    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    

    //camera setup
    let cam = Camera::new();
    let mut rng = rand::thread_rng();
    
    for (x,y,pixel) in buffer.enumerate_pixels_mut() {   

        let mut pixel_color = Color::new(0.0,0.0,0.0);

        for _ in 0..SAMPLES_PER_PIXEL {
            let random_u: f64 = rng.gen();
            let random_v: f64 = rng.gen();

            let u = ((x as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
            let v = 1.0  - (((y as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64));
            let ray = cam.get_ray(u, v);

            pixel_color += ray_color(&ray,&world);
        }
        
        pixel_color /= SAMPLES_PER_PIXEL as f64;
        *pixel = Rgb(pixel_color.to_rgb());
    }

     
    match buffer.save("antialiasing.png") {
        Err(e) => eprintln!("Error writing file {}",e),
        Ok(()) => println!("Saving Done!")
    }    
}