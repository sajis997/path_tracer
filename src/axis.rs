#![allow(unused)]
use std::fmt::{Display, Formatter, Result};
use std::ops::{Index, IndexMut};

struct MyType<T>(T);

pub enum Axis {
    /// Index of the X axis.
    X = 0,

    /// Index of the Y axis.
    Y = 1,

    /// Index of the Z axis.
    Z = 2,
}

impl Display for Axis {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f,
                "{}",
                 match *self {
                     Axis::X => "x",
                     Axis::Y => "y",
                     Axis::Z => "z"
                 })
    }
}
