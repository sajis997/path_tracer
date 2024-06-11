use glam::Vec3;

use crate::utils::util::Point3;

pub struct Ray {
    origo: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray {
            origo: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origo
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origo + t * self.dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::*;

    macro_rules! assert_vec3_equal {
        ($expected:expr, $actual:expr) => {
            let tolerance = 0.0001;
            assert_approx_eq!($expected.x, $actual.x, tolerance);
            assert_approx_eq!($expected.y, $actual.y, tolerance);
            assert_approx_eq!($expected.z, $actual.z, tolerance);
        };
    }

    #[test]
    fn at_distance() {
        let ray = Ray::new(Point3::new(1.0, 1.0, 1.0), Vec3::new(3.0, 4.0, 0.0));
        let position = ray.at(5.0);
        let expected = Vec3::new(16.0, 21.0, 1.0);
        assert_vec3_equal!(expected, position);
    }
}
