
use glam::Vec3;
use crate::hit::{Hit, HitRecord};
use crate::material::Scatter;
use crate::ray::Ray;
use crate::util::Point3;
use crate::aabb::Aabb;

pub struct Sphere<M: Scatter> {
    center: Point3,
    radius: f32,
    mat: M,
}

impl<M: Scatter> Sphere<M> {
    pub fn new(center: Point3, radius: f32, mat: M) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl<M: Scatter> Hit for Sphere<M> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length().powi(2);
        let half_b = oc.dot(r.direction());
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
            mat: &self.mat,
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
        };

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }
    
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<Aabb> {
        //return the bounding box of the sphere 
        // that is already calculated in the constructor
        let r = Vec3::new(self.radius, self.radius, self.radius);
        let box_sphere = Aabb::new(self.center - r, self.center + r);        
        Some(box_sphere)
    }
}
