use math::Vec3;
use math::Plane3D;
use math::IsNan;

use std::fmt;
use std::ops::{Neg, Add, Mul, Div};

#[derive(Copy, Clone)]
pub struct Ray3D<A> {
    pub origin: Vec3<A>,
    pub direction: Vec3<A>
}

impl<A: fmt::Display> fmt::Display for Ray3D<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Ray3D {{ origin: {}, direction {} }})", self.origin, self.direction)
    }
}

impl<A> Ray3D<A> {

    pub fn intersection_time_with_plane(&self, plane: &Plane3D<A>) -> Option<A>
    where A: IsNan + Neg<Output=A> + Add<A,Output=A> + Mul<A,Output=A> + Div<A,Output=A> + Copy
    {
        // n.(ro + rd.t) + d = 0
        // n.(ro + rd.t) = -d
        // n.ro + n.rd.t = -d
        // n.rd.t = -d - n.ro
        // t = (-d - n.ro) / (n.rd)
        // t = -(d + n.ro) / (n.rd)
        let t = -(plane.d + plane.normal.dot(&self.origin)) / plane.normal.dot(&self.direction);
        if t.is_nan() {
            None
        } else {
            Some(t)
        }
    }

    pub fn point_from_time(&self, time: &A) -> Vec3<A>
    where A: Copy + Add<A,Output=A> + Mul<A,Output=A>
    {
        return self.origin.clone() + self.direction.clone() * time.clone();
    }
}
