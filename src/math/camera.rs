use math::Axes3D;
use math::Mat4;
use math::Ray3D;
use math::Vec2;
use math::Vec3;
use math::Vec4;
use math::Zero;
use math::One;
use math::Projection;

use std::ops::{Neg, Add, Sub, Mul, Div};
use std::convert;
use std::cmp::PartialEq;
use std::fmt;

#[derive(Copy, Clone)]
pub struct Camera<A> where A: Copy {
    pub axes: Axes3D<A>,
    pub projection: Projection<A>
}

impl<A: Copy> Camera<A> {
    pub fn screen_to_world(&self, screen_pt: &Vec2<A>) -> Ray3D<A>
    where A: fmt::Display + Copy + PartialEq + Zero + One + Neg<Output=A> + Add<A,Output=A> + Sub<A,Output=A> + Mul<A,Output=A> + Div<A,Output=A> + convert::From<i8>
    {
        let ray = self.projection.unproject_point(screen_pt);
        let result = Ray3D {
            origin: self.axes.point_from_space(&ray.origin),
            direction: self.axes.vector_from_space(&ray.direction)
        };
        println!("Camera::screen_to_world(self, {}) = {}", screen_pt, result);
        result
    }

    pub fn world_to_screen(&self, world_pt: &Vec3<A>) -> Vec2<A>
    where A: Copy + PartialEq + Zero + One + Neg<Output=A> + Add<A,Output=A> + Sub<A,Output=A> + Mul<A,Output=A> + Div<A,Output=A> + convert::From<i8>
    {
        let world_pt2 = self.axes.point_to_space(world_pt);
        self.projection.project_point(&world_pt2)
    }
}
