use sodium::Stream;
use math::Vec2;

pub struct InputDriver {
    key_pressed: Stream<u32>,
    key_released: Stream<u32>,
    mouse_moved: Stream<Vec2<f64>>,
    mouse_exited: Stream<()>,
    mouse_pressed: Stream<u32>,
    mouse_released: Stream<u32>
}
