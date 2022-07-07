use crate::vec::*;
use crate::ray::*;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) {
        Sphere {
            center:center,
            radius:radius
        }
    }

    pub fn hit(&self, ray: &Ray) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        discriminant > 0.0
    }
}