pub use math::One;
pub use math::Ray3D;
pub use math::Vec2;
pub use math::Vec3;
pub use math::Zero;

use std::ops::{Neg, Add, Sub, Mul, Div};

#[derive(Copy, Clone)]
pub enum Projection<A> {
    Perspective {
        left: A,
        right: A,
        bottom: A,
        top: A,
        near: A
    },
    Orthogonal {
        left: A,
        right: A,
        bottom: A,
        top: A
    }
}

impl<A> Projection<A> {
    pub fn project_point(&self, world_pt: &Vec3<A>) -> Vec2<A>
    where A: Copy + One + Add<A,Output=A> + Sub<A,Output=A> + Mul<A,Output=A> + Div<A,Output=A>
    {
        match self {
            &Projection::Perspective {left, right, bottom, top, near} => {
                let half = A::one() / (A::one() + A::one());
                let p2 = world_pt.clone() * (near / world_pt.z);
                Vec2 {
                    x: p2.x + half * (left + right),
                    y: p2.y + half * (bottom + top)
                }
            },
            &Projection::Orthogonal {left, right, bottom, top} => {
                let half = A::one() / (A::one() + A::one());
                Vec2 {
                    x: world_pt.x + half * (left + right),
                    y: world_pt.y + half * (bottom + top)
                }
            }
        }
    }

    pub fn unproject_point(&self, screen_pt: &Vec2<A>) -> Ray3D<A>
    where A: Copy + Zero + One + Neg<Output=A> + Add<A,Output=A> + Sub<A,Output=A> + Mul<A,Output=A> + Div<A,Output=A>
    {
        match self {
            &Projection::Perspective {left, right, bottom, top, near} => {
                let half = A::one() / (A::one() + A::one());
                Ray3D {
                    origin: Vec3 {
                        x: A::zero(),
                        y: A::zero(),
                        z: A::zero()
                    },
                    direction: Vec3 {
                        x: screen_pt.x - half * (left + right),
                        y: screen_pt.y - half * (bottom + top),
                        z: -near
                    }
                }
            },
            &Projection::Orthogonal {left, right, bottom, top} => {
                let half = A::one() / (A::one() + A::one());
                Ray3D {
                    origin: Vec3 {
                        x: screen_pt.x - half * (left + right),
                        y: screen_pt.y - half * (bottom + top),
                        z: A::zero()
                    },
                    direction: Vec3 {
                        x: A::zero(),
                        y: A::zero(),
                        z: A::one()
                    }
                }
            }
        }
    }
}
