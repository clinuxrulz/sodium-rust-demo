use std::ops::{Add, Sub, Mul, Div, Neg};
use std::fmt;
use std::convert;
use std::cmp::PartialOrd;

use math::Zero;
use math::One;
use math::Vec3;
use math::Sqrt;
use math::Acos;
use math::Cos;
use math::Sin;

#[derive(Copy, Clone)]
pub struct Quaternion<A> {
    pub w: A,
    pub x: A,
    pub y: A,
    pub z: A
}

impl<A: fmt::Display> fmt::Display for Quaternion<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i + {}j + {}k", self.w, self.x, self.y, self.z)
    }
}

impl<A: Add> Add for Quaternion<A> {
    type Output = Quaternion<<A as Add>::Output>;

    fn add(self, other: Quaternion<A>) -> Quaternion<<A as Add>::Output> {
        Quaternion::<<A as Add>::Output> {
            w: self.w + other.w,
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl<A: Sub> Sub for Quaternion<A> {
    type Output = Quaternion<<A as Sub>::Output>;

    fn sub(self, other: Quaternion<A>) -> Quaternion<<A as Sub>::Output> {
        Quaternion::<<A as Sub>::Output> {
            w: self.w - other.w,
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl<A: Copy + Mul> Mul<A> for Quaternion<A> {
    type Output = Quaternion<<A as Mul>::Output>;

    fn mul(self, other: A) -> Quaternion<<A as Mul>::Output>
    {
        Quaternion::<<A as Mul>::Output> {
            w: self.w * other,
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl<A: Copy + Add<A,Output=A> + Sub<A,Output=A> + Mul<A,Output=A>> Mul for Quaternion<A> {
    type Output = Quaternion<A>;

    fn mul(self, other: Quaternion<A>) -> Quaternion<A>
    {
        /*
         (w0 + x0*i + y0*j + z0*k)*(w1 + x1*i + y1*j + z1*k)
         = w0*w1 - x0*x1 - y0*y1 - z0*z1
         + (w0*x1 + w1*x0 + y0*z1 - y1*z0)*i
         + (w0*y1 + w1*y0 + z0*x1 - z1*x0)*j
         + (w0*z1 + w1*z0 + x0*y1 - x1*y0)*k
        */
        Quaternion::<A> {
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
            x: self.w * other.x + other.w * self.x + self.y * other.z - other.y * self.z,
            y: self.w * other.y + other.w * self.y + self.z * other.x - other.z * self.x,
            z: self.w * other.z + other.w * self.z + self.x * other.y - other.x * self.y
        }
    }
}

impl Quaternion<f64> {
    pub fn of_vec_angle(v: Vec3<f64>, a: f64) -> Quaternion<f64> {
        let v2 = v.normalize();
        let a2 = a.to_radians();
        let c = (0.5 * a2).cos();
        let s = (0.5 * a2).sin();
        Quaternion {
            w: c,
            x: v2.x * s,
            y: v2.y * s,
            z: v2.z * s
        }
    }
}

impl<A: Neg<Output=A>> Neg for Quaternion<A> {
    type Output = Quaternion<A>;

    fn neg(self) -> Quaternion<A> {
        Quaternion::of(-self.w, -self.x, -self.y, -self.z)
    }
}

impl<A> Quaternion<A> {

    pub fn identity() -> Quaternion<A>
    where A: Zero + One
    {
        Quaternion {
            w: A::one(),
            x: A::zero(),
            y: A::zero(),
            z: A::zero()
        }
    }

    pub fn of(w: A, x: A, y: A, z: A) -> Quaternion<A> {
        Quaternion {
            w: w,
            x: x,
            y: y,
            z: z
        }
    }

    pub fn of_uv(u: &Vec3<A>, v: &Vec3<A>) -> Quaternion<A>
    where A: Copy + Zero + One + Add<Output=A> + Sub<Output=A> + Mul<Output=A> + Div<Output=A> + Sqrt<Output=A> + PartialOrd
    {
        Quaternion::of_uvw(u, v, &u.cross(v))
    }

    pub fn of_vw(v: &Vec3<A>, w: &Vec3<A>) -> Quaternion<A>
    where A: Copy + Zero + One + Add<Output=A> + Sub<Output=A> + Mul<Output=A> + Div<Output=A> + Sqrt<Output=A> + PartialOrd
    {
        Quaternion::of_uvw(&v.cross(w), v, w)
    }

    pub fn of_wu(w: &Vec3<A>, u: &Vec3<A>) -> Quaternion<A>
    where A: Copy + Zero + One + Add<Output=A> + Sub<Output=A> + Mul<Output=A> + Div<Output=A> + Sqrt<Output=A> + PartialOrd
    {
        Quaternion::of_uvw(u, &w.cross(u), w)
    }

    fn of_uvw(u: &Vec3<A>, v: &Vec3<A>, w: &Vec3<A>) -> Quaternion<A>
    where A: Copy + Zero + One + Add<Output=A> + Sub<Output=A> + Mul<Output=A> + Div<Output=A> + Sqrt<Output=A> + PartialOrd
    {
        let zero = A::zero();
        let one = A::one();
        let two = one + one;
        let four = two + two;
        let quater = one / four;
        let m00 = u.x;
        let m10 = u.y;
        let m20 = u.z;
        let m01 = v.x;
        let m11 = v.y;
        let m21 = v.z;
        let m02 = w.x;
        let m12 = w.y;
        let m22 = w.z;
        let tr = m00 + m11 + m22;
        let qw;
        let qx;
        let qy;
        let qz;
        if tr > zero {
            let s = (tr + one).sqrt() * two; // S=4*qw
            qw = quater * s;
            qx = (m21 - m12) / s;
            qy = (m02 - m20) / s;
            qz = (m10 - m01) / s;
        } else if (m00 > m11) & (m00 > m22) {
            let s = (one + m00 - m11 - m22).sqrt() * two; // S=4*qx
            qw = (m21 - m12) / s;
            qx = quater * s;
            qy = (m01 + m10) / s;
            qz = (m02 + m20) / s;
        } else if m11 > m22 {
            let s = (one + m11 - m00 - m22).sqrt() * two; // S=4*qy
            qw = (m02 - m20) / s;
            qx = (m01 + m10) / s;
            qy = quater * s;
            qz = (m12 + m21) / s;
        } else {
            let s = (one + m22 - m00 - m11).sqrt() * two; // S=4*qz
            qw = (m10 - m01) / s;
            qx = (m02 + m20) / s;
            qy = (m12 + m21) / s;
            qz = quater * s;
        }
        Quaternion::of(qw, qx, qy, qz)
    }

    pub fn u(&self) -> Vec3<A>
    where A: Copy + Zero + One + Neg<Output=A> + Add<A,Output=A> + Sub<A,Output=A> + Mul<A,Output=A>
    {
        self.rotate_vec3(&Vec3::of(A::one(), A::zero(), A::zero()))
    }

    pub fn v(&self) -> Vec3<A>
    where A: Copy + Zero + One + Neg<Output=A> + Add<A,Output=A> + Sub<A,Output=A> + Mul<A,Output=A>
    {
        self.rotate_vec3(&Vec3::of(A::zero(), A::one(), A::zero()))
    }

    pub fn w(&self) -> Vec3<A>
    where A: Copy + Zero + One + Neg<Output=A> + Add<A,Output=A> + Sub<A,Output=A> + Mul<A,Output=A>
    {
        self.rotate_vec3(&Vec3::of(A::zero(), A::zero(), A::one()))
    }

    pub fn conjugate(self) -> Quaternion<A>
    where
    A: Neg<Output=A>
    {
        Quaternion::<A> {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }

    pub fn rotate_vec3(self, other: &Vec3<A>) -> Vec3<A>
    where
    A: Copy + Zero + Neg<Output=A> + Add<A,Output=A> + Sub<A,Output=A> + Mul<A,Output=A>
    {
        let v = Quaternion::<A> {
            w: A::zero(),
            x: other.x,
            y: other.y,
            z: other.z
        };
        let v2 = self * v * self.conjugate();
        return Vec3::<A> {
            x: v2.x,
            y: v2.y,
            z: v2.z
        };
    }

    pub fn length(&self) -> A
    where A: Copy + Add<Output=A> + Mul<Output=A> + Sqrt<Output=A>
    {
        (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Quaternion<A>
    where A: Copy + Add<Output=A> + Mul<Output=A> + Div<Output=A> + Sqrt<Output=A>
    {
        let length = self.length();
        Quaternion::of(self.w / length, self.x / length, self.y / length, self.z / length)
    }

    pub fn dot(&self, other: &Quaternion<A>) -> A
    where A: Copy + Add<Output=A> + Mul<Output=A>
    {
        self.w * other.w + self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn slerp(self, v1: &Quaternion<A>, t: A) -> Quaternion<A>
    where A: Copy + Zero + One + Neg<Output=A> + Add<Output=A> + Sub<Output=A> + Mul<Output=A> + Div<Output=A> + Sqrt<Output=A> + convert::From<f32> + Acos<Output=A> + Cos<Output=A> + Sin<Output=A> + PartialOrd
    {

        // Only unit quaternions are valid rotations.
        // Normalize to avoid undefined behavior.
        let v0 = self.normalize();
        let mut v1 = v1.normalize();

        // Compute the cosine of the angle between the two vectors.
        let mut dot = v0.dot(&v1);

        let dot_threshold: A = 0.9995f32.into();
        if dot > dot_threshold {
            // If the inputs are too close for comfort, linearly interpolate
            // and normalize the result.

            return (v0 + (v1 - v0) * t).normalize();
        }

        // If the dot product is negative, the quaternions
        // have opposite handed-ness and slerp won't take
        // the shorter path. Fix by reversing one quaternion.
        if dot < A::zero() {
            v1 = -v1;
            dot = -dot;
        }

        // Robustness: Stay within domain of acos()
        if dot < -A::one() {
            dot = -A::one();
        } else if dot > A::one() {
            dot = A::one();
        }

        let theta_0 = dot.acos();  // theta_0 = angle between input vectors
        let theta = theta_0*t;    // theta = angle between v0 and result

        // { v0, v2 } is now an orthonormal basis
        let v2 = (v1 - v0 * dot).normalize();

        return v0*theta.cos() + v2*theta.sin();
    }
}
