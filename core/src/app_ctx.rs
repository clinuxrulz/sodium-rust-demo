use drivers::DisplayDriver;
use drivers::InputDriver;
use math::Vec2;

use sodium::SodiumCtx;
use sodium::StreamSink;
use sodium::IsStream;

pub struct AppCtx {
    pub sodium_ctx: SodiumCtx,
    pub input_driver: InputDriver,
    pub display_driver: Box<DisplayDriver>,
    // Input Driver StreamSinks
    ss_input_tick: StreamSink<u32>,
    ss_input_key_pressed: StreamSink<u32>,
    ss_input_key_released: StreamSink<u32>,
    ss_input_mouse_moved: StreamSink<Vec2<f64>>,
    ss_input_mouse_exited: StreamSink<()>,
    ss_input_mouse_pressed: StreamSink<u32>,
    ss_input_mouse_released: StreamSink<u32>
}

impl AppCtx {
    pub fn new<DD: DisplayDriver + 'static>(display_driver: DD) -> AppCtx {
        let mut sodium_ctx = SodiumCtx::new();
        let ss_input_tick = sodium_ctx.new_stream_sink();
        let ss_input_key_pressed = sodium_ctx.new_stream_sink();
        let ss_input_key_released = sodium_ctx.new_stream_sink();
        let ss_input_mouse_moved = sodium_ctx.new_stream_sink();
        let ss_input_mouse_exited = sodium_ctx.new_stream_sink();
        let ss_input_mouse_pressed = sodium_ctx.new_stream_sink();
        let ss_input_mouse_released = sodium_ctx.new_stream_sink();
        let input_driver = InputDriver {
            tick: ss_input_tick.stream(),
            key_pressed: ss_input_key_pressed.stream(),
            key_released: ss_input_key_released.stream(),
            mouse_moved: ss_input_mouse_moved.stream(),
            mouse_exited: ss_input_mouse_exited.stream(),
            mouse_pressed: ss_input_mouse_pressed.stream(),
            mouse_released: ss_input_mouse_released.stream()
        };
        AppCtx {
            sodium_ctx: sodium_ctx,
            input_driver,
            display_driver: Box::new(display_driver),
            ss_input_tick,
            ss_input_key_pressed,
            ss_input_key_released,
            ss_input_mouse_moved,
            ss_input_mouse_exited,
            ss_input_mouse_pressed,
            ss_input_mouse_released
        }
    }

    pub fn input_tick(&mut self, ms: u32) {
        self.ss_input_tick.send(&ms);
    }

    pub fn input_key_pressed(&mut self, key_code: u32) {
        self.ss_input_key_pressed.send(&key_code);
    }

    pub fn input_key_released(&mut self, key_code: u32) {
        self.ss_input_key_released.send(&key_code);
    }

    pub fn input_mouse_moved(&mut self, pos: Vec2<f64>) {
        self.ss_input_mouse_moved.send(&pos);
    }

    pub fn input_mouse_exited(&mut self) {
        self.ss_input_mouse_exited.send(&());
    }

    pub fn input_mouse_pressed(&mut self, buttons: u32) {
        self.ss_input_mouse_pressed.send(&buttons);
    }

    pub fn input_mouse_released(&mut self, buttons: u32) {
        self.ss_input_mouse_released.send(&buttons);
    }
}
