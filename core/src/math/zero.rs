pub trait Zero {
    fn zero() -> Self;
}

impl Zero for f32 {
    fn zero() -> f32 { 0.0 }
}

impl Zero for f64 {
    fn zero() -> f64 { 0.0 }
}
