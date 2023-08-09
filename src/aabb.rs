use glam::Vec3;
use std::fmt;

use crate::axis::Axis;
use crate::util::Point3;

pub struct Aabb {
    min: Point3, // minimum coordinate
    max: Point3, // maximum coordinate
}

impl fmt::Display for Aabb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Min bound: {}; Max bound: {}", self.min, self.max)
    }
}

// a trait implemented by things which can be bounded by an AABB
pub trait Bounded {
    fn aabb(&self) -> Aabb;
}

/*
    in the following snippet we are making a blanket implementation on
    a generic type T , that implements Bounded. Therefore, we use behavior
    guaranteed by the Bounded type, to produce a Aabb representation, to our
    advantage.

    This is a great way to remove redundancy in code by reducing the need
    to repeat the code for different types with similar functionality

    we can call the method aabb defined by the Bounded trait on any type
*/
impl<T: Bounded> Bounded for &T {
    fn aabb(&self) -> Aabb {
        T::aabb(self)
    }
}

impl<T: Bounded> Bounded for &mut T {
    fn aabb(&self) -> Aabb {
        T::aabb(self)
    }
}

impl<T: Bounded> Bounded for Box<T> {
    fn aabb(&self) -> Aabb {
        T::aabb(self)
    }
}

impl Aabb {
    // creates a new Aabb with given bounds
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }

    // creates an empty bounding box
    pub fn empty() -> Aabb {
        Aabb {
            min: Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
            max: Point3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.min.x > self.max.x || self.min.y > self.max.y || self.min.z > self.max.z
    }

    pub fn contains(&self, p: &Point3) -> bool {
        p.x >= self.min.x
            && p.x <= self.max.x
            && p.y >= self.min.y
            && p.y <= self.max.y
            && p.z >= self.min.z
            && p.z <= self.max.z
    }

    pub fn approx_contains_eps(&self, p: &Point3, epsilon: f32) -> bool {
        p.x - self.min.x > -epsilon
            && p.x - self.max.x < epsilon
            && p.y - self.min.y > epsilon
            && p.y - self.max.y < epsilon
            && p.z - self.min.z > epsilon
            && p.z - self.max.z < epsilon
    }

    pub fn approx_contains_aabb_eps(&self, other: &Aabb, epsilon: f32) -> bool {
        self.approx_contains_eps(&other.min, epsilon)
            && self.approx_contains_eps(&other.max, epsilon)
    }

    pub fn relative_eq(&self, other: &Aabb, epsilon: f32) -> bool {
        f32::abs(self.min.x - other.min.x) < epsilon
            && f32::abs(self.min.y - other.min.y) < epsilon
            && f32::abs(self.min.z - other.min.z) < epsilon
            && f32::abs(self.max.x - other.max.x) < epsilon
            && f32::abs(self.max.y - other.max.y) < epsilon
            && f32::abs(self.max.z - other.max.z) < epsilon
    }

    pub fn include(&self, other: &Aabb) -> Aabb {
        Aabb::new(
            Point3::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            Point3::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        )
    }

    pub fn include_mut(&mut self, other: &Aabb) {
        self.min = Point3::new(
            self.min.x.min(other.min.x),
            self.min.y.min(other.min.y),
            self.min.z.min(other.min.z),
        );

        self.max = Point3::new(
            self.max.x.max(other.max.x),
            self.max.y.max(other.max.y),
            self.max.z.max(other.max.z),
        );
    }

    pub fn grow(&self, other: &Point3) -> Aabb {
        Aabb::new(
            Point3::new(
                self.min.x.min(other.x),
                self.min.y.min(other.y),
                self.min.z.min(other.z),
            ),
            Point3::new(
                self.max.x.max(other.x),
                self.max.y.max(other.y),
                self.max.z.max(other.z),
            ),
        )
    }

    pub fn grow_mut(&mut self, other: &Point3) {
        self.min = Point3::new(
            self.min.x.min(other.x),
            self.min.y.min(other.y),
            self.min.z.min(other.z),
        );

        self.max = Point3::new(
            self.max.x.max(other.x),
            self.max.y.max(other.y),
            self.max.z.max(other.z),
        );
    }

    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    pub fn center(&self) -> Point3 {
        self.min + (self.size() / 2.0)
    }

    pub fn surface_area(&self) -> f32 {
        let size = self.size();
        2.0 * (size.x * size.y + size.x * size.z + size.y * size.z)
    }

    pub fn volume(&self) -> f32 {
        let size = self.size();
        size.x * size.y * size.z
    }

    pub fn largest_axis(&self) -> Axis {
        let size = self.size();

        if size.x > size.y && size.x > size.z {
            Axis::X
        } else if size.y > size.z {
            Axis::Y
        } else {
            Axis::Z
        }
    }
}

#[cfg(test)]
mod tests {
    /*
        the tests module is a redular module that follows the usual visibility rules.
        tests module is an inner module , we need to bring the code under the test
        in the outer module into the scope of the inner module.
    */
    use super::*;

    #[test]
    fn initiation() {
        let aabb = Aabb::new(Point3::new(-1.0, -1.0, -1.0), Point3::new(1.0, 1.0, 1.0));

        assert_eq!(aabb.min.x, -1.0);
        assert_eq!(aabb.max.x, 1.0)
    }

    #[test]
    fn relative_eq_test() {
        let aabb = Aabb::new(Point3::new(-1.0, -1.0, -1.0), Point3::new(1.0, 1.0, 1.0));
        let point_barely_outside_min = Point3::new(-1.000_000_1, -1.000_000_1, -1.000_000_1);
        let point_barely_outside_max = Point3::new(1.000_000_1, 1.000_000_1, 1.000_000_1);

        let other = Aabb::new(point_barely_outside_min, point_barely_outside_max);

        assert!(aabb.relative_eq(&other, 0.00001));
    }

    #[test]
    fn check_empty() {
        let aabb = Aabb::empty();

        let min = &aabb.min;
        let max = &aabb.max;

        let x = rand::random();
        let y = rand::random();
        let z = rand::random();

        //an empty Aabb should not contain it
        assert!(x < min.x && y < min.y && z < min.z);
        assert!(max.x < x && max.y < y && max.z < z);
    }

    #[test]
    fn containment_test() {
        let aabb = Aabb::new(Point3::new(-1.0, -1.0, -1.0), Point3::new(1.0, 1.0, 1.0));

        let point_inside = Point3::new(0.125, -0.25, 0.5);
        let point_outside = Point3::new(1.0, -2.0, 4.0);

        assert!(aabb.contains(&point_inside));
        assert!(!aabb.contains(&point_outside));
    }

    #[test]
    fn approx_containment_test() {
        let aabb = Aabb::new(Point3::new(-1.0, -1.0, -1.0), Point3::new(1.0, 1.0, 1.0));
        let point_barely_outside = Point3::new(1.0000_0000_1, -1.0000_0000_1, 1.0000_0000_001);
        let point_outside = Point3::new(1.0, -2.0, 4.0);

        // assert!(aabb.approx_contains_eps(&point_barely_outside));
        //assert!(!aabb.approx_contains_eps(&point_outside));
    }

    #[test]
    fn inclusion_test() {
        let aabb1 = Aabb::new(Point3::new(-101.0, 0.0, 0.0), Point3::new(-100.0, 1.0, 1.0));

        let aabb2 = Aabb::new(Point3::new(100.0, 0.0, 0.0), Point3::new(101.0, 1.0, 1.0));

        let joint = aabb1.include(&aabb2);

        let point_inside_aabb1 = Point3::new(-100.5, 0.5, 0.5);
        let point_inside_aabb2 = Point3::new(100.5, 0.5, 0.5);
        let point_inside_joint = Point3::new(0.0, 0.5, 0.5);

        assert!(aabb1.contains(&point_inside_aabb1));
        assert!(!aabb1.contains(&point_inside_aabb2));
        assert!(!aabb1.contains(&point_inside_joint));

        assert!(!aabb2.contains(&point_inside_aabb1));
        assert!(aabb2.contains(&point_inside_aabb2));
        assert!(!aabb2.contains(&point_inside_joint));

        assert!(joint.contains(&point_inside_aabb1));
        assert!(joint.contains(&point_inside_aabb2));
        assert!(joint.contains(&point_inside_joint));
    }

    #[test]
    fn inclusion_test_mut() {
        let size = Vec3::new(1.0, 1.0, 1.0);
        let aabb_pos = Point3::new(-101.0, 0.0, 0.0);
        let mut aabb = Aabb::new(aabb_pos, aabb_pos + size);

        let other_pos = Point3::new(100.0, 0.0, 0.0);
        let other_aabb = Aabb::new(other_pos, other_pos + size);

        let point_inside_aabb = aabb_pos + size / 2.0;
        let point_inside_other = other_pos + size / 2.0;
        let point_inside_joint = Point3::new(0.0, 0.0, 0.0) + size / 2.0;

        assert!(aabb.contains(&point_inside_aabb));
        assert!(!aabb.contains(&point_inside_other));
        assert!(!aabb.contains(&point_inside_joint));

        assert!(!other_aabb.contains(&point_inside_aabb));
        assert!(other_aabb.contains(&point_inside_other));
        assert!(!other_aabb.contains(&point_inside_joint));

        aabb.include_mut(&other_aabb);

        assert!(aabb.contains(&point_inside_aabb));
        assert!(aabb.contains(&point_inside_other));
        assert!(aabb.contains(&point_inside_joint));
    }

    #[test]
    fn grow_test() {
        let point1 = Point3::new(0.0, 0.0, 0.0);
        let point2 = Point3::new(1.0, 1.0, 1.0);
        let point3 = Point3::new(2.0, 2.0, 2.0);

        let aabb = Aabb::empty();
        assert!(!aabb.contains(&point1));

        let aabb1 = aabb.grow(&point1);
        assert!(aabb1.contains(&point1));

        let aabb2 = aabb.grow(&point2);
        assert!(aabb2.contains(&point2));
        assert!(!aabb2.contains(&point3));
    }

    #[test]
    fn grow_mut_test() {
        let point1 = Point3::new(0.0, 0.0, 0.0);
        let point2 = Point3::new(1.0, 1.0, 1.0);
        let point3 = Point3::new(2.0, 2.0, 2.0);

        let mut aabb = Aabb::empty();
        assert!(!aabb.contains(&point1));

        aabb.grow_mut(&point1);
        assert!(aabb.contains(&point1));

        aabb.grow_mut(&point2);
        assert!(aabb.contains(&point2));
        assert!(!aabb.contains(&point3));
    }

    #[test]
    fn size_test() {
        let aabb = Aabb::new(Point3::new(-1.0, -1.0, -1.0), Point3::new(1.0, 1.0, 1.0));

        let size = aabb.size();

        assert!(size.x == 2.0 && size.y == 2.0 && size.z == 2.0);
    }

    #[test]
    fn center_test() {
        let min = Point3::new(41.0, 41.0, 41.0);
        let max = Point3::new(43.0, 43.0, 43.0);

        let aabb = Aabb::new(min, max);
        let center = aabb.center();

        assert!(center.x == 42.0 && center.y == 42.0 && center.z == 42.0);
    }

    #[test]
    fn is_empty_test() {
        let empty_aabb = Aabb::empty();

        assert!(empty_aabb.is_empty());

        let min = Point3::new(41.0, 41.0, 41.0);
        let max = Point3::new(43.0, 43.0, 43.0);

        let aabb = Aabb::new(min, max);
        assert!(!aabb.is_empty());
    }

    #[test]
    fn surface_area_test() {
        let min = Point3::new(41.0, 41.0, 41.0);
        let max = Point3::new(43.0, 43.0, 43.0);

        let aabb = Aabb::new(min, max);
        let surface_area = aabb.surface_area();
        assert!(surface_area == 24.0);
    }

    #[test]
    fn volume_test() {
        let min = Point3::new(41.0, 41.0, 41.0);
        let max = Point3::new(43.0, 43.0, 43.0);

        let aabb = Aabb::new(min, max);
        let volume = aabb.volume();
        assert!(volume == 8.0);
    }

    #[test]
    fn largest_axis_test() {
        let min = Point3::new(-100.0, 0.0, 0.0);
        let max = Point3::new(100.0, 0.0, 0.0);

        let aabb = Aabb::new(min, max);
        let axis = aabb.largest_axis();

        //assert!(axis == Axis::X);
    }
}
