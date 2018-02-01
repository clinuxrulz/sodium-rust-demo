pub trait Cos {
    type Output;

    fn cos(self) -> Self::Output;
}

impl Cos for f32 {
    type Output = f32;

    fn cos(self) -> f32 {
        self.cos()
    }
}

impl Cos for f64 {
    type Output = f64;

    fn cos(self) -> f64 {
        self.cos()
    }
}
