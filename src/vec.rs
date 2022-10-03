use rand::prelude::*;
use std::fmt;
use std::fmt::Display;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Range, Sub, SubAssign,
};

#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3], // Vec3 is an array of consecutive 3 elements in the array of type f64
}

pub type Point3 = Vec3; // give a new name to the existing type
pub type Color = Vec3; // give a new name to the existing type

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self {
            e: [e0, e1, e2], // a private field
        }
    }

    pub fn x(&self) -> f64 {
        self.e[0] // get the first element in the array
    }

    pub fn y(&self) -> f64 {
        self.e[1] // get the second element in the array
    }

    pub fn z(&self) -> f64 {
        self.e[2] // get the third element in the array
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * other.e[2] - self.e[2] * other.e[1], 
                self.e[2] * other.e[0] - self.e[0] * other.e[2],
                self.e[0] * other.e[1] - self.e[1] * other.e[0],
            ],
        }
    }

    pub fn length(&self) -> f64 {
        self.dot(&self).sqrt()
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }

    pub fn to_rgb(&self) -> [u8; 3] {
        fn f(num: f64) -> u8 {
            if num < 0.0 {
                0
            } else if num >= 1.0 {
                255.99 as u8
            } else {
                (num * 256.0) as u8
            }
        }
        [f(self.e[0]), f(self.e[1]), f(self.e[2])]
    }

    pub fn gamma_correction(&self, samples_per_pixel: u32) -> [u8; 3] {
        [
            (256.0
                * (self.e[0] / (samples_per_pixel as f64 /* explicit conversion */))
                    .sqrt()
                    .clamp(0.0, 0.999)) as u8 /* explicit conversion */,
            (256.0
                * (self.e[1] / (samples_per_pixel as f64 /* explicit conversion */))
                    .sqrt()
                    .clamp(0.0, 0.999)) as u8 /* explicit conversion */,
            (256.0
                * (self.e[2] / (samples_per_pixel as f64 /* explicit conversion */))
                    .sqrt()
                    .clamp(0.0, 0.999)) as u8 /* explicit conversion */,
        ]
    }

    pub fn random(r: Range<f64>) -> Vec3 {
        let mut rng = rand::thread_rng();

        Vec3 {
            e: [
                rng.gen_range(r.clone()),
                rng.gen_range(r.clone()),
                rng.gen_range(r.clone()),
            ],
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let v = Vec3::random(-1.0..1.0);
            if v.length() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -1.0 * in_unit_sphere
        }
    }

    pub fn format_color(&self) -> String {
        format!(
            "{} {} {}",
            (255.999 * self[0]) as u64,
            (255.999 * self[1]) as u64,
            (255.999 * self[2]) as u64
        )
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.e[0], self.e[1], self.e[2])
    }
}

//implement Trits for the Vec3 type
//negation operator
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

//immutable index
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}

//mutable index
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.e[index]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = Vec3 {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = Vec3 {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = Vec3 {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self * other.e[0], self * other.e[1], self * other.e[2]],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs],
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Vec3 {
            e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::*;

    macro_rules! assert_vec3_equal {
        ($expected:expr, $actual:expr) => {
            let tolerance = 0.0001;
            assert_approx_eq!($expected.e[0], $actual.e[0], tolerance);
            assert_approx_eq!($expected.e[1], $actual.e[1], tolerance);
            assert_approx_eq!($expected.e[2], $actual.e[2], tolerance);
        };
    }

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
        let result = vector1.dot(&vector2);

        assert_approx_eq!(32.0, result, 0.001);
    }
}
