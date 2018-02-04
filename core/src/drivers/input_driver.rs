use sodium::Stream;
use math::Vec2;

pub struct InputDriver {
    pub tick: Stream<f64>, // f64 represents milliseconds since last tick
    pub key_pressed: Stream<u32>,
    pub key_released: Stream<u32>,
    pub mouse_moved: Stream<Vec2<f64>>,
    pub mouse_exited: Stream<()>,
    pub mouse_pressed: Stream<u32>,
    pub mouse_released: Stream<u32>
}
