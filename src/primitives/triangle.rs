
use crate::hit::Hit;
use crate::ray::Ray;
use crate::utils::util::Point3;
use crate::material::Scatter;
use crate::hit::HitRecord;
use crate::utils::aabb::Aabb;

pub struct Triangle<M: Scatter> {
    vertices: [Point3; 3],
    material: M,
}

impl <M: Scatter> Triangle<M>{
    pub fn new(vertices: [Point3; 3], material: M) -> Self {
        Self {
            vertices,
            material,
        }
    }
}

impl<M: Scatter> Hit for Triangle<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let edge1 = self.vertices[1] - self.vertices[0];
        let edge2 = self.vertices[2] - self.vertices[0];
        let h = ray.direction().cross(edge2);
        let a = edge1.dot(h);
    
        if a > -std::f32::EPSILON && a < std::f32::EPSILON {
            return None; // This ray is parallel to this triangle.
        }
    
        let f = 1.0 / a;
        let s = ray.origin() - self.vertices[0];
        let u = f * s.dot(h);
    
        if u < 0.0 || u > 1.0 {
            return None;
        }
    
        let q = s.cross(edge1);
        let v = f * ray.direction().dot(q);
    
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
    
        // At this stage we can compute t to find out where the intersection point is on the line.
        let t = f * edge2.dot(q);
    
        if t > std::f32::EPSILON {
            // ray intersection
            let p = ray.origin() + t * ray.direction();

            // calculate the face normal
            let normal = edge1.cross(edge2).normalize();

            let mut rec = HitRecord 
            {   p: p, 
                normal: normal, 
                mat: &self.material, 
                t: t, 
                front_face: false };

            rec.set_face_normal(ray, normal);
            
            Some(rec)
        } else {
            // This means that there is a line intersection but not a ray intersection.
            None
        }
    }
    
    fn bounding_box(&self) -> Option<Aabb> {
        
        // find the min and max of the x, y, and z coordinates of the triangle
        // add/sub epsilon to avoid infinitely thin bounding boxes
        let min = Point3::new(
            self.vertices[0].x.min(self.vertices[1].x.min(self.vertices[2].x)) - std::f32::EPSILON,
            self.vertices[0].y.min(self.vertices[1].y.min(self.vertices[2].y)) - std::f32::EPSILON,
            self.vertices[0].z.min(self.vertices[1].z.min(self.vertices[2].z)) - std::f32::EPSILON,
        );

        let max = Point3::new(
            self.vertices[0].x.max(self.vertices[1].x.max(self.vertices[2].x)) + std::f32::EPSILON,
            self.vertices[0].y.max(self.vertices[1].y.max(self.vertices[2].y)) + std::f32::EPSILON,
            self.vertices[0].z.max(self.vertices[1].z.max(self.vertices[2].z)) + std::f32::EPSILON,
        );

        Some(Aabb::new(min, max))       
    }

    // get the centroid of the triangle
    fn centroid(&self) -> Point3 {
        (self.vertices[0] + self.vertices[1] + self.vertices[2]) / 3.0
    }    
}