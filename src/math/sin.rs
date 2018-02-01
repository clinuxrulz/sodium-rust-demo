pub trait Sin {
    type Output;

    fn sin(self) -> Self::Output;
}

impl Sin for f32 {
    type Output = f32;

    fn sin(self) -> f32 {
        self.sin()
    }
}

impl Sin for f64 {
    type Output = f64;

    fn sin(self) -> f64 {
        self.sin()
    }
}
