use std::ops::{Add, Sub, Mul};
use std::fmt;
use std::convert;

#[derive(Copy, Clone)]
pub struct Vec2<A> {
    pub x: A,
    pub y: A
}

impl<A: fmt::Display> fmt::Display for Vec2<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<A: Add<A,Output=A>> Add for Vec2<A> {
    type Output = Vec2<A>;

    fn add(self, other: Vec2<A>) -> Vec2<A> {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl<A: Sub<A,Output=A>> Sub for Vec2<A> {
    type Output = Vec2<A>;

    fn sub(self, other: Vec2<A>) -> Vec2<A> {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl<A> Vec2<A> {

    pub fn of(x: A, y: A) -> Vec2<A> {
        Vec2 { x: x, y: y }
    }

    pub fn dot(&self, other: &Vec2<A>) -> A
    where
    A: Copy + Add<A,Output=A> + Mul<A,Output=A>
    {
        return self.x * other.x + self.y * other.y;
    }

    pub fn cross(&self, other: &Vec2<A>) -> A
    where
    A: Copy + Sub<A,Output=A> + Mul<A,Output=A>
    {
        return self.x * other.y - self.y * other.x;
    }

    pub fn length_squared(&self) -> A
    where
    A: Copy + Add<A,Output=A> + Mul<A,Output=A>
    {
        return self.x * self.x + self.y * self.y;
    }

    pub fn length(&self) -> f64
    where
    A: Copy + Add<A,Output=A> + Mul<A,Output=A> + convert::Into<f64>
    {
        self.length_squared().into().sqrt()
    }
}
