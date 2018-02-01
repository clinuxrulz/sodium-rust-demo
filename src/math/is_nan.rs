pub trait IsNan {
    fn is_nan(&self) -> bool;
}

impl IsNan for f32 {
    fn is_nan(&self) -> bool { f32::is_nan(self.clone()) }
}

impl IsNan for f64 {
    fn is_nan(&self) -> bool { f64::is_nan(self.clone()) }
}
