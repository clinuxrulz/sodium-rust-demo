use std::ops::{Neg, Add, Sub, Mul, Div};
use std::cmp::PartialOrd;
use std::convert;

use math::Axes3D;
use math::Camera;
use math::One;
use math::Acos;
use math::Cos;
use math::Sin;
use math::Zero;
use math::Sqrt;

pub struct CameraTransition<A> where A: Copy {
    from_camera: Camera<A>,
    to_camera: Camera<A>,
    start_time: A,
    period: A
}

impl<A: Copy> CameraTransition<A> {
    pub fn of(from_camera: Camera<A>, to_camera: Camera<A>, start_time: A, period: A) -> CameraTransition<A> {
        CameraTransition {
            from_camera: from_camera,
            to_camera: to_camera,
            start_time: start_time,
            period: period
        }
    }

    pub fn camera_at_time(&self, time: A) -> (Camera<A>,bool)
    where
    A: Copy + Zero + One + Sqrt<Output=A> + Neg<Output=A> + Add<Output=A> + Sub<Output=A> + Mul<Output=A> + Div<Output=A> + Acos<Output=A> + Cos<Output=A> + Sin<Output=A> + PartialOrd + convert::From<f32>
    {
        let t = (time - self.start_time) / self.period;
        let done = t >= A::one();
        if done {
            return (self.to_camera,done);
        }
        let from_origin = self.from_camera.axes.origin;
        let to_origin = self.to_camera.axes.origin;
        let from_quaternion = self.from_camera.axes.orientation;
        let to_quaternion = self.to_camera.axes.orientation;
        (
            Camera {
                axes: Axes3D {
                    origin: (to_origin - from_origin) * t + from_origin,
                    orientation: from_quaternion.slerp(&to_quaternion, t)
                },
                projection: self.to_camera.projection
            },
            done
        )
    }
}
