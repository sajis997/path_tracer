use std::rc::Rc;

use crate::hit::{Hit, HitRecord};
use crate::material::Scatter;
use crate::ray::Ray;
use crate::vec::{Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Scatter>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Rc<dyn Scatter>) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length().powi(2);
        let half_b = oc.dot(&r.direction());
        let c = oc.length().powi(2) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        //find the nearest root that lies in the acceptable range
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b - sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            mat: self.mat.clone(),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
        };

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }
}
