use math::One;
use math::Zero;
use math::Vec4;

use std::ops::{Neg, Add, Sub, Mul, Div};
use std::fmt;
use std::convert;
use std::cmp::PartialEq;

#[derive(Copy, Clone)]
pub struct Mat4<A> where A: Copy {
    pub cells: [A; 16]
}

impl<A: Zero + Copy + Add<A,Output=A> + Mul<A,Output=A>> Mul for Mat4<A> {
    type Output = Mat4<A>;

    fn mul(self, other: Mat4<A>) -> Mat4<A> {
        let mut result = [A::zero(); 16];
        for i in 0..3 {
            for j in 0..3 {
                let mut sum = A::zero();
                for k in 0..3 {
                    sum = sum + self.cells[(i<<2) + k] * other.cells[(k<<2) + j];
                }
                result[(i<<2) + j] = sum;
            }
        }
        Mat4 { cells: result }
    }
}

impl<A: Copy + Add<A,Output=A> + Mul<A,Output=A>> Mul<Vec4<A>> for Mat4<A> {
    type Output = Vec4<A>;

    fn mul(self, other: Vec4<A>) -> Vec4<A> {
        Vec4 {
            x: self.cells[0] * other.x + self.cells[1] * other.y + self.cells[2] * other.z + self.cells[3] * other.w,
            y: self.cells[4] * other.x + self.cells[5] * other.y + self.cells[6] * other.z + self.cells[7] * other.w,
            z: self.cells[8] * other.x + self.cells[9] * other.y + self.cells[10] * other.z + self.cells[11] * other.w,
            w: self.cells[12] * other.x + self.cells[13] * other.y + self.cells[14] * other.z + self.cells[15] * other.w
        }
    }
}

impl<A: Copy> Mat4<A> {
    pub fn identity() -> Mat4<A>
    where A: Zero + One
    {
        Mat4 {
            cells: [
                A::one(), A::zero(), A::zero(), A::zero(),
                A::zero(), A::one(), A::zero(), A::zero(),
                A::zero(), A::zero(), A::one(), A::zero(),
                A::zero(), A::zero(), A::zero(), A::one()
            ]
        }
    }

    pub fn inverse(&self) -> Option<Mat4<A>>
    where A: Copy + PartialEq + Zero + One + Neg<Output=A> + Add<A,Output=A> + Sub<A,Output=A> + Mul<A,Output=A> + Div<A,Output=A>
    {
        let m = &self.cells;
        let mut inv = [
            // inv[0]
            m[5]  * m[10] * m[15] -
            m[5]  * m[11] * m[14] -
            m[9]  * m[6]  * m[15] +
            m[9]  * m[7]  * m[14] +
            m[13] * m[6]  * m[11] -
            m[13] * m[7]  * m[10],

            // inv[1]
            -m[1]  * m[10] * m[15] +
            m[1]  * m[11] * m[14] +
            m[9]  * m[2] * m[15] -
            m[9]  * m[3] * m[14] -
            m[13] * m[2] * m[11] +
            m[13] * m[3] * m[10],

            // inv[2]
            m[1]  * m[6] * m[15] -
            m[1]  * m[7] * m[14] -
            m[5]  * m[2] * m[15] +
            m[5]  * m[3] * m[14] +
            m[13] * m[2] * m[7] -
            m[13] * m[3] * m[6],

            // inv[3]
            -m[1] * m[6] * m[11] +
            m[1] * m[7] * m[10] +
            m[5] * m[2] * m[11] -
            m[5] * m[3] * m[10] -
            m[9] * m[2] * m[7] +
            m[9] * m[3] * m[6],

            // inv[4]
            -m[4]  * m[10] * m[15] +
            m[4]  * m[11] * m[14] +
            m[8]  * m[6]  * m[15] -
            m[8]  * m[7]  * m[14] -
            m[12] * m[6]  * m[11] +
            m[12] * m[7]  * m[10],

            // inv[5]
            m[0]  * m[10] * m[15] -
            m[0]  * m[11] * m[14] -
            m[8]  * m[2] * m[15] +
            m[8]  * m[3] * m[14] +
            m[12] * m[2] * m[11] -
            m[12] * m[3] * m[10],

            // inv[6]
            -m[0]  * m[6] * m[15] +
            m[0]  * m[7] * m[14] +
            m[4]  * m[2] * m[15] -
            m[4]  * m[3] * m[14] -
            m[12] * m[2] * m[7] +
            m[12] * m[3] * m[6],

            // inv[7]
            m[0] * m[6] * m[11] -
            m[0] * m[7] * m[10] -
            m[4] * m[2] * m[11] +
            m[4] * m[3] * m[10] +
            m[8] * m[2] * m[7] -
            m[8] * m[3] * m[6],

            // inv[8]
            m[4]  * m[9] * m[15] -
            m[4]  * m[11] * m[13] -
            m[8]  * m[5] * m[15] +
            m[8]  * m[7] * m[13] +
            m[12] * m[5] * m[11] -
            m[12] * m[7] * m[9],

            // inv[9]
            -m[0]  * m[9] * m[15] +
            m[0]  * m[11] * m[13] +
            m[8]  * m[1] * m[15] -
            m[8]  * m[3] * m[13] -
            m[12] * m[1] * m[11] +
            m[12] * m[3] * m[9],

            // inv[10]
            m[0]  * m[5] * m[15] -
            m[0]  * m[7] * m[13] -
            m[4]  * m[1] * m[15] +
            m[4]  * m[3] * m[13] +
            m[12] * m[1] * m[7] -
            m[12] * m[3] * m[5],

            // inv[11]
            -m[0] * m[5] * m[11] +
            m[0] * m[7] * m[9] +
            m[4] * m[1] * m[11] -
            m[4] * m[3] * m[9] -
            m[8] * m[1] * m[7] +
            m[8] * m[3] * m[5],

            // inv[12]
            -m[4]  * m[9] * m[14] +
            m[4]  * m[10] * m[13] +
            m[8]  * m[5] * m[14] -
            m[8]  * m[6] * m[13] -
            m[12] * m[5] * m[10] +
            m[12] * m[6] * m[9],

            // inv[13]
            m[0]  * m[9] * m[14] -
            m[0]  * m[10] * m[13] -
            m[8]  * m[1] * m[14] +
            m[8]  * m[2] * m[13] +
            m[12] * m[1] * m[10] -
            m[12] * m[2] * m[9],

            // inv[14]
            -m[0]  * m[5] * m[14] +
            m[0]  * m[6] * m[13] +
            m[4]  * m[1] * m[14] -
            m[4]  * m[2] * m[13] -
            m[12] * m[1] * m[6] +
            m[12] * m[2] * m[5],

            // inv[15]
            m[0] * m[5] * m[10] -
            m[0] * m[6] * m[9] -
            m[4] * m[1] * m[10] +
            m[4] * m[2] * m[9] +
            m[8] * m[1] * m[6] -
            m[8] * m[2] * m[5]
        ];

        let mut det = m[0] * inv[0] + m[1] * inv[4] + m[2] * inv[8] + m[3] * inv[12];

        if det == A::zero() {
            return None;
        }

        det = A::one() / det;

        for i in 0..15 {
            inv[i] = inv[i] * det;
        }

        Some(Mat4 { cells: inv })
    }

    pub fn frustum(
        left: A,
        right: A,
        bottom: A,
        top: A,
        nearVal: A,
        farVal: A
    ) -> Mat4<A>
    where A: Copy + Zero + One + Neg<Output=A> + Add<A,Output=A> + Sub<A,Output=A> + Mul<A,Output=A> + Div<A,Output=A>
    {
        let two = A::one() + A::one();
        Mat4 {
            cells: [
                two * nearVal / (right - left), A::zero(), (left + right) / (right - left), A::zero(),
                A::zero(), two * nearVal / (top - bottom), (bottom + top) / (top - bottom), A::zero(),
                A::zero(), A::zero(), -(farVal + nearVal) / (farVal - nearVal), -two * farVal * nearVal / (farVal - nearVal),
                A::zero(), A::zero(), -A::one(), A::zero()
            ]
        }
    }

    pub fn ortho(
        left: A,
        right: A,
        bottom: A,
        top: A,
        nearVal: A,
        farVal: A
    ) -> Mat4<A>
    where A: Copy + Zero + One + Neg<Output=A> + Add<A,Output=A> + Sub<A,Output=A> + Mul<A,Output=A> + Div<A,Output=A>
    {
        let two = A::one() + A::one();
        Mat4 {
            cells: [
                two / (right - left), A::zero(), A::zero(), -(right + left) / (right - left),
                A::zero(), two / (top - bottom), A::zero(), -(top + bottom) / (top - bottom),
                A::zero(), A::zero(), -two / (farVal - nearVal), -(farVal + nearVal) / (farVal - nearVal),
                A::zero(), A::zero(), A::zero(), A::one()
            ]
        }
    }
}
