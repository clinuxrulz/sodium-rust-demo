use std::ops::{Add, Sub, Mul, Neg};

use math::Vec3;
use math::Quaternion;
use math::Zero;

#[derive(Copy, Clone)]
pub struct Axes3D<A> {
    pub origin: Vec3<A>,
    pub orientation: Quaternion<A>
}

impl<A> Axes3D<A> {

    pub fn point_from_space(&self, point: &Vec3<A>) -> Vec3<A>
    where
    A: Copy + Zero + Add<A,Output=A> + Sub<A,Output=A> + Mul<A,Output=A> + Neg<Output=A>
    {
        self.orientation.rotate_vec3(point) + self.origin
    }

    pub fn point_to_space(&self, point: &Vec3<A>) -> Vec3<A>
    where
    A: Copy + Zero + Add<A,Output=A> + Sub<A,Output=A> + Mul<A,Output=A> + Neg<Output=A>
    {
        let point2 = point.clone() - self.origin;
        self.orientation.conjugate().rotate_vec3(&point2)
    }

    pub fn vector_from_space(&self, vector: &Vec3<A>) -> Vec3<A>
    where
    A: Copy + Zero + Add<A,Output=A> + Sub<A,Output=A> + Mul<A,Output=A> + Neg<Output=A>
    {
        self.orientation.rotate_vec3(vector)
    }

    pub fn vector_to_space(&self, vector: &Vec3<A>) -> Vec3<A>
    where
    A: Copy + Zero + Add<A,Output=A> + Sub<A,Output=A> + Mul<A,Output=A> + Neg<Output=A>
    {
        self.orientation.conjugate().rotate_vec3(vector)
    }
}
