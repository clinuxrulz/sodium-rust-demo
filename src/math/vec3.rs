use std::ops::{Add, Sub, Mul};
use std::fmt;
use std::convert;

#[derive(Copy, Clone)]
pub struct Vec3<A> {
    pub x: A,
    pub y: A,
    pub z: A
}

impl<A: fmt::Display> fmt::Display for Vec3<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<A: Add> Add for Vec3<A> {
    type Output = Vec3<<A as Add>::Output>;

    fn add(self, other: Vec3<A>) -> Vec3<<A as Add>::Output> {
        Vec3::<<A as Add>::Output> { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl<A: Sub> Sub for Vec3<A> {
    type Output = Vec3<<A as Sub>::Output>;

    fn sub(self, other: Vec3<A>) -> Vec3<<A as Sub>::Output> {
        Vec3::<<A as Sub>::Output> { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl<A: Copy + Mul<A,Output=A>> Mul<A> for Vec3<A> {
    type Output = Vec3<A>;

    fn mul(self, other: A) -> Vec3<A> {
        Vec3 {
            x: self.x * other.clone(),
            y: self.y * other.clone(),
            z: self.z * other
        }
    }
}

impl Vec3<f64> {

    pub fn normalize(&self) -> Vec3<f64> {
        return self.clone() * (1.0 / self.length());
    }
}

impl<A> Vec3<A> {

    pub fn of(x: A, y: A, z: A) -> Vec3<A> {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn dot(&self, other: &Vec3<A>) -> A
    where
    A: Copy + Add<A, Output = A> + Mul<A, Output = A>
    {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn cross(&self, other: &Vec3<A>) -> Vec3<A>
    where
    A: Copy + Sub<A, Output = A> + Mul<A, Output = A>
    {
        return Vec3::<A> {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        };
    }

    pub fn length_squared(&self) -> A
    where
    A: Copy + Add<A, Output = A> + Mul<A, Output = A>
    {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn length(&self) -> f64
    where
    A: Copy + Add<A, Output = A> + Mul<A, Output = A> + convert::Into<f64>
    {
        self.length_squared().into().sqrt()
    }
}
