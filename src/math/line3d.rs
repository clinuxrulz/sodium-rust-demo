use std::ops::{Add, Sub, Mul};
use std::fmt;
use math::Vec3;
use std::convert;

#[derive(Copy, Clone)]
pub struct Line3D<A> {
    pub v1: Vec3<A>,
    pub v2: Vec3<A>
}

impl<A: fmt::Display> fmt::Display for Line3D<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line3D {{ v1: {}, v2: {} }}", self.v1, self.v2)
    }
}

impl<A> Line3D<A> {
    pub fn length(&self) -> f64
    where
    A: Copy + Sub<A,Output=A> + Add<A,Output=A> + Mul<A,Output=A> + convert::Into<f64> {
        return (self.v2 - self.v1).length();
    }
}
