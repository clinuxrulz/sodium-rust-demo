use sodium::Stream;
use math::Vec2;

pub struct InputDriver {
    pub s_tick: Stream<f64>, // f64 represents milliseconds since last tick
    pub s_key_pressed: Stream<u32>,
    pub s_key_released: Stream<u32>,
    pub s_mouse_moved: Stream<Vec2<f64>>,
    pub s_mouse_exited: Stream<()>,
    pub s_mouse_pressed: Stream<u32>,
    pub s_mouse_released: Stream<u32>
}
