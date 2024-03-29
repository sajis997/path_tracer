use glam::Vec3;

use crate::material::Scatter;
use crate::ray::Ray;
use crate::util::Point3;
use crate::aabb::Aabb;

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: &'a dyn Scatter,
    pub t: f32,
    pub front_face: bool,
}


impl<'a> HitRecord<'a> {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            (-1.0) * outward_normal
        };
    }
} 

/*
    the following vector is of type World - is a triat object
    it is a stand-in for any type inside a Box that implements
    the Hit trait.

    A vector of Box<dyn Hit> is a vector of any type that implements the Hit trait.
 */
pub type World = Vec<Box<dyn Hit>>;

impl Hit for World {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest_so_far = t_max;

        for object in self {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                tmp_rec = Some(rec);
            }
        }

        tmp_rec
    }
    
    fn bounding_box(&self) -> Option<Aabb> {
        if self.is_empty() {
            return None;
        }

        let mut output_box : Option<Aabb> = None;
        for object in self {
            if let Some(tmp_box) = object.bounding_box() {
                output_box = match output_box {
                    Some(output_box) => Some(output_box.include(&tmp_box)),
                    None => Some(tmp_box),
                };
            } else {
                return None;
            }
        }

        output_box
    }
}


/*
    Sync marker triats are used to mark types that are safe to share between threads.
*/
pub trait Hit : Sync {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;

    fn bounding_box(&self) -> Option<Aabb>;
}
