use crate::ray::Ray;
use crate::vec::{Point3, Vec3};

/*
    Develop the camera incrementaly
    Let us allow an adjustable field of view (fov). This is the angle we see through the portal
    Our image is not square, the fov is different horizontally and vertiacally.
    Initially, the rays came from the origin and heading to the z= -1 plane. We could make it z = -2 plane,
    as long as we made h a ratio to thant distance.

    positioning and orienting the camera

    to get the arbitrary viewpoint, let's name the points we care about.
    1. Position the camera with  - lookfrom(..)
    2. Where do the camera do we look at - lookat(..)
    3. We also need a way to specify the roll, sideways tilt of the camera
*/
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Camera {
        // Vertical field-of-view in degrees
        let theta = std::f64::consts::PI / 180.0 * vfov;
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let cw = (lookfrom - lookat).normalized();
        let cu = vup.cross(&cw).normalized();
        let cv = cw.cross(&cu);

        let h = viewport_width * cu;
        let v = viewport_height * cv;

        let llc = lookfrom - h / 2.0 - v / 2.0 - cw;

        Camera {
            origin: lookfrom,
            horizontal: h,
            vertical: v,
            lower_left_corner: llc,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
