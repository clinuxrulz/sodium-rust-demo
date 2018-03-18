use ecs::EcsContext;
use ecs::EcsManager;
use ecs::components::Axes3DComponent;
use ecs::components::CircleComponent;
use ecs::components::PlayerComponent;
use ecs::systems::DisplaySystem;
use ecs::systems::InputSystem;
use drivers::DiagDriver;
use drivers::Log;
use drivers::DisplayDriver;
use drivers::InputDriver;
use math::Axes3D;
use math::Vec2;
use math::Vec3;
use math::Quaternion;

use sodium::SodiumCtx;
use sodium::StreamSink;
use sodium::IsStream;

use std::cell::RefCell;
use std::rc::Rc;

pub struct AppCtx {
    pub sodium_ctx: SodiumCtx,
    pub diag_driver: Rc<RefCell<DiagDriver>>,
    pub input_driver: Rc<RefCell<InputDriver>>,
    pub display_driver: Rc<RefCell<DisplayDriver>>,
    // ecs
    ecs_manager: Rc<RefCell<EcsManager>>,
    // Input Driver StreamSinks
    ss_input_tick: StreamSink<f64>,
    ss_input_key_pressed: StreamSink<u32>,
    ss_input_key_released: StreamSink<u32>,
    ss_input_mouse_moved: StreamSink<Vec2<f64>>,
    ss_input_mouse_exited: StreamSink<()>,
    ss_input_mouse_pressed: StreamSink<u32>,
    ss_input_mouse_released: StreamSink<u32>
}

impl AppCtx {
    pub fn new<DIAG_D: DiagDriver + 'static, DD: DisplayDriver + 'static>(diag_driver: DIAG_D, display_driver: DD) -> AppCtx {
        let mut sodium_ctx = SodiumCtx::new();
        let ss_input_tick = sodium_ctx.new_stream_sink();
        let ss_input_key_pressed = sodium_ctx.new_stream_sink();
        let ss_input_key_released = sodium_ctx.new_stream_sink();
        let ss_input_mouse_moved = sodium_ctx.new_stream_sink();
        let ss_input_mouse_exited = sodium_ctx.new_stream_sink();
        let ss_input_mouse_pressed = sodium_ctx.new_stream_sink();
        let ss_input_mouse_released = sodium_ctx.new_stream_sink();
        let input_driver = Rc::new(RefCell::new(InputDriver {
            s_tick: ss_input_tick.stream(),
            s_key_pressed: ss_input_key_pressed.stream(),
            s_key_released: ss_input_key_released.stream(),
            s_mouse_moved: ss_input_mouse_moved.stream(),
            s_mouse_exited: ss_input_mouse_exited.stream(),
            s_mouse_pressed: ss_input_mouse_pressed.stream(),
            s_mouse_released: ss_input_mouse_released.stream()
        }));
        let ecs_manager = Rc::new(RefCell::new(EcsManager::new()));
        let diag_driver = Rc::new(RefCell::new(diag_driver));
        let display_driver = Rc::new(RefCell::new(display_driver));
        {
            let mut ecs_manager2 = ecs_manager.borrow_mut();
            ecs_manager2.add_system(
                InputSystem::new(
                    input_driver.clone(),
                    ecs_manager.clone()
                )
            );
            ecs_manager2.add_system(
                DisplaySystem::new(
                    display_driver.clone()
                )
            );
            ecs_manager2.transaction(|ecs_manager2: &mut EcsManager| {
                let player = ecs_manager2.create_entity();
                ecs_manager2.set_component(&player, CircleComponent { radius: 10.0 });
                ecs_manager2.set_component(&player, PlayerComponent {});
                ecs_manager2.set_component(&player, Axes3DComponent {
                    axes: Axes3D {
                        origin: Vec3::of(50.0, 50.0, 0.0),
                        orientation: Quaternion::identity()
                    }
                });
            });
            diag_driver.borrow_mut().log("Created player entity");
        }
        AppCtx {
            sodium_ctx: sodium_ctx,
            diag_driver,
            input_driver,
            display_driver: display_driver,
            ecs_manager: ecs_manager,
            ss_input_tick,
            ss_input_key_pressed,
            ss_input_key_released,
            ss_input_mouse_moved,
            ss_input_mouse_exited,
            ss_input_mouse_pressed,
            ss_input_mouse_released
        }
    }

    pub fn input_tick(&mut self, ms: f64) {
        self.ss_input_tick.send(&ms);
    }

    pub fn input_key_pressed(&mut self, key_code: u32) {
        self.diag_driver.borrow_mut().log(format!("key pressed {}", key_code));
        self.ss_input_key_pressed.send(&key_code);
    }

    pub fn input_key_released(&mut self, key_code: u32) {
        self.diag_driver.borrow_mut().log(format!("key released {}", key_code));
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
