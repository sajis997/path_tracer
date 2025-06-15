use crate::hit::{HitRecord, World};
use crate::ray::Ray;
use crate::utils::aabb::Aabb;

pub trait Accelerator {
    // Method to build the acceleration structure
    fn build(&mut self, world: &World);

    // Method to check for intersection with a ray
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

}