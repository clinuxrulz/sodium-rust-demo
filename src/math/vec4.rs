use std::ops::{Add, Sub, Mul};
use std::fmt;
use std::convert;

#[derive(Copy, Clone)]
pub struct Vec4<A> {
    pub x: A,
    pub y: A,
    pub z: A,
    pub w: A
}

impl<A: fmt::Display> fmt::Display for Vec4<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl<A: Add<A,Output=A>> Add for Vec4<A> {
    type Output = Vec4<A>;

    fn add(self, other: Vec4<A>) -> Vec4<A> {
        Vec4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        }
    }
}

impl<A: Sub<A,Output=A>> Sub for Vec4<A> {
    type Output = Vec4<A>;

    fn sub(self, other: Vec4<A>) -> Vec4<A> {
        Vec4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w
        }
    }
}

impl<A> Vec4<A> {
    pub fn dot(&self, other: &Vec4<A>) -> A
    where
    A: Copy + Add<A, Output = A> + Mul<A, Output = A>
    {
        return self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w;
    }

    pub fn length_squared(&self) -> A
    where
    A: Copy + Add<A, Output = A> + Mul<A, Output = A>
    {
        return self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
    }

    pub fn length(&self) -> f64
    where
    A: Copy + Add<A, Output = A> + Mul<A, Output = A> + convert::Into<f64>
    {
        self.length_squared().into().sqrt()
    }
}
