

mod vec; // to use Vec3 in the program, add a reference with mod keyword
mod ray;
mod hit;
mod sphere;

use image::{RgbImage,ImageBuffer,Rgb};
use vec::{Vec3,Color,Point3};
use ray::Ray;
use sphere::Sphere;
use hit::{Hit,World};

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
    
    //image plane
    let mut buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH as u32,IMAGE_HEIGHT as u32);

    //world
    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    

    //camera setup
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0,0.0,0.0);
    let horizontal = Vec3::new(viewport_width,0.0,0.0);
    let vertical = Vec3::new(0.0,viewport_height,0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0,0.0,focal_length); 
            
     for (x,y,pixel) in buffer.enumerate_pixels_mut() {        
            let u = x as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = 1.0 - (y as f64 / (IMAGE_HEIGHT - 1) as f64);
            
            let ray = Ray::new(origin,lower_left_corner + u * horizontal + v * vertical - origin);

            let color = ray_color(&ray,&world).to_rgb();

            *pixel = Rgb(color);
    }

     
    match buffer.save("world.png") {
        Err(e) => eprintln!("Error writing file {}",e),
        Ok(()) => println!("Saving Done!")
    }    
}