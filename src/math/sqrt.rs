pub trait Sqrt {
    type Output;

    fn sqrt(self) -> Self::Output;
}

impl Sqrt for f32 {
    type Output = f32;

    fn sqrt(self) -> f32 {
        self.sqrt()
    }
}

impl Sqrt for f64 {
    type Output = f64;

    fn sqrt(self) -> f64 {
        self.sqrt()
    }
}
