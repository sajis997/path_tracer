use glam::Vec3;
use rand::prelude::*;
use std::ops::Range;

#[derive(Clone, Copy)]
pub struct Util;

pub type Point3 = Vec3; // give a new name to the existing type
pub type Color = Vec3; // give a new name to the existing type

impl Util {
    /// Convert the Vec3 to Color Array
    pub fn to_rgb(vec: &Vec3) -> [u8; 3] {
        fn f(num: f32) -> u8 {
            if num < 0.0 {
                0
            } else if num >= 1.0 {
                255.99 as u8
            } else {
                (num * 256.0) as u8
            }
        }
        [f(vec.x), f(vec.y), f(vec.z)]
    }

    /// Return the Gamma Corrected Image as Color Array
    pub fn gamma_correction(vec: &Vec3, samples_per_pixel: u32) -> [u8; 3] {
        [
            (256.0
                * (vec.x / (samples_per_pixel as f32/* explicit conversion */))
                    .sqrt()
                    .clamp(0.0, 0.999)) as u8, /* explicit conversion */
            (256.0
                * (vec.y / (samples_per_pixel as f32/* explicit conversion */))
                    .sqrt()
                    .clamp(0.0, 0.999)) as u8, /* explicit conversion */
            (256.0
                * (vec.z / (samples_per_pixel as f32/* explicit conversion */))
                    .sqrt()
                    .clamp(0.0, 0.999)) as u8, /* explicit conversion */
        ]
    }

    /// Generate Vec3 by generating random number
    pub fn random(r: Range<f32>) -> Vec3 {
        let mut rng = rand::thread_rng();

        Vec3::new(
            rng.gen_range(r.clone()),
            rng.gen_range(r.clone()),
            rng.gen_range(r.clone()),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let v = Util::random(-1.0..1.0);
            if v.length() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(*normal) > 0.0 {
            in_unit_sphere
        } else {
            -1.0 * in_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();

        loop {
            let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length() < 1.0 {
                return p;
            }
        }
    }

    pub fn near_zero(vec: &Vec3) -> bool {
        const EPS: f32 = 1.0e-8;
        vec.x.abs() < EPS && vec.y.abs() < EPS && vec.z.abs() < EPS
    }

    pub fn format_color(color: &Color) -> String {
        format!(
            "{} {} {}",
            (255.999 * color.x) as u64,
            (255.999 * color.y) as u64,
            (255.999 * color.z) as u64
        )
    }

    pub fn reflect(incoming_vec: &Vec3, n: &Vec3) -> Vec3 {
        *incoming_vec - 2.0 * incoming_vec.dot(*n) * *n
    }

    pub fn refract(incoming_vec: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = ((-1.0) * *incoming_vec).dot(*n).min(1.0);
        let r_out_perp = etai_over_etat * (*incoming_vec + cos_theta * *n);
        let r_out_parallel = -(1.0 - r_out_perp.length().powi(2)).abs().sqrt() * *n;

        r_out_perp + r_out_parallel
    }
}

/*
    the convention is to create a module name tests in each file to contain
    the test functions and to annotate the module with cfg(test)
*/
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

    // annotated with test attribute. Attributes are metadata about pieces of Rust Code.
    // this annotation indicates that this is a test function.
    #[test]
    fn addition() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(4.0, 5.0, 6.0);
        let result = vec1 + vec2;
        let expected = Vec3::new(5.0, 7.0, 9.0);

        assert_vec3_equal!(expected, result);
    }

    #[test]
    fn assign_addition() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let mut vec2 = Vec3::new(4.0, 5.0, 6.0);

        vec2 += vec1;
        let expected = Vec3::new(5.0, 7.0, 9.0);

        assert_vec3_equal!(expected, vec2);
    }

    #[test]
    fn subtration() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(4.0, 5.0, 6.0);
        let result = vec1 - vec2;
        let expected = Vec3::new(-3.0, -3.0, -3.0);

        assert_vec3_equal!(expected, result);
    }

    #[test]
    fn assign_subtration() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let mut vec2 = Vec3::new(4.0, 5.0, 6.0);

        vec2 -= vec1;
        let expected = Vec3::new(3.0, 3.0, 3.0);

        assert_vec3_equal!(expected, vec2);
    }

    #[test]
    fn multiplication_scalar() {
        let vec = Vec3::new(2.0, 3.2, 2.2);

        let result = vec * 2.0;

        let expected = Vec3::new(4.0, 6.4, 4.4);

        assert_vec3_equal!(expected, result);
    }

    #[test]
    fn scalar_multiplication() {
        let vec = Vec3::new(2.0, 3.2, 2.2);

        let result = 2.0 * vec;

        let expected = Vec3::new(4.0, 6.4, 4.4);

        assert_vec3_equal!(expected, result);
    }

    #[test]
    fn assign_multiplication_scalar() {
        let mut vec = Vec3::new(2.0, 3.2, 2.2);

        vec *= 2.0;

        let expected = Vec3::new(4.0, 6.4, 4.4);

        assert_vec3_equal!(expected, vec);
    }

    #[test]
    fn multiplication_vec3() {
        let vec1 = Vec3::new(2.0, 3.2, 2.2);
        let vec2 = Vec3::new(4.0, 5.2, 6.2);

        let result = vec1 * vec2;
        let expected = Vec3::new(8.0, 16.64, 13.64);

        assert_vec3_equal!(result, expected);
    }

    #[test]
    fn assign_multiplication_vec3() {
        let vec1 = Vec3::new(2.0, 3.2, 2.2);
        let mut vec2 = Vec3::new(4.0, 5.2, 6.2);

        vec2 *= vec1;

        let expected = Vec3::new(8.0, 16.64, 13.64);

        assert_vec3_equal!(vec2, expected);
    }

    #[test]
    fn division_by_scalar() {
        let vec = Vec3::new(4.5, 6.7, 8.8);
        let scalar = 3.4;

        let result = vec / scalar;

        let expected = Vec3::new(1.3235, 1.9705, 2.5882);

        assert_vec3_equal!(result, expected);
    }

    #[test]
    fn assign_division_by_scalar() {
        let mut vec = Vec3::new(4.5, 6.7, 8.8);
        let scalar = 3.4;

        vec /= scalar;

        let expected = Vec3::new(1.3235, 1.9705, 2.5882);

        assert_vec3_equal!(vec, expected);
    }

    #[test]
    fn negation() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);

        let result = -vec1;
        let expected = Vec3::new(-1.0, -2.0, -3.0);

        assert_vec3_equal!(result, expected);
    }

    #[test]
    fn vec_dot() {
        let vector1 = Vec3::new(1.0, 2.0, 3.0);
        let vector2 = Vec3::new(1.0, 5.0, 7.0);
        let result = vector1.dot(vector2);

        assert_approx_eq!(32.0, result, 0.001);
    }
}
