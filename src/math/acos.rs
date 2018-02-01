pub trait Acos {
    type Output;

    fn acos(self) -> Self::Output;
}

impl Acos for f32 {
    type Output = f32;

    fn acos(self) -> f32 {
        self.acos()
    }
}

impl Acos for f64 {
    type Output = f64;

    fn acos(self) -> f64 {
        self.acos()
    }
}
