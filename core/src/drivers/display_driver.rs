pub trait DisplayDriver {
    fn add_circle(&mut self, x: f64, y: f64, r: f64) -> u32;
    fn move_circle(&mut self, id: u32, x: f64, y: f64);
    fn remove(&mut self, id: u32);
}
