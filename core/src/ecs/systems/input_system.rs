use drivers::InputDriver;
use ecs::EcsContext;
use ecs::EcsEventContext;
use ecs::EcsManager;
use ecs::Entity;
use ecs::IsComponent;
use ecs::IsEcsEvent;
use ecs::System;
use ecs::components::Axes3DComponent;
use ecs::components::PlayerComponent;
use ecs::events::ComponentSetEvent;
use ecs::events::ComponentUnsetEvent;
use math::Axes3D;
use math::Vec3;
use math::Zero;

use sodium::IsStream;
use sodium::Listener;
use sodium::Stream;
use sodium::IsStreamOption;

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

pub struct InputSystem {
    ecs_manager: Rc<RefCell<EcsManager>>,
    player_set: Rc<RefCell<HashSet<Entity>>>,
    keep_alive: Vec<Listener>
}

impl InputSystem {
    pub fn new(input_driver: Rc<RefCell<InputDriver>>, ecs_manager: Rc<RefCell<EcsManager>>) -> InputSystem {
        let input_driver = input_driver.borrow();
        const LEFT_KEY_CODE: u32 = 37;
        const RIGHT_KEY_CODE: u32 = 39;
        const DOWN_KEY_CODE: u32 = 40;
        const UP_KEY_CODE: u32 = 38;
        let key_code_pressed =
            |key_code:u32|
                input_driver
                    .s_key_pressed
                    .filter(move |k:&u32| *k == key_code)
                    .map_to(());
        let key_code_released =
            |key_code:u32|
                input_driver
                    .s_key_released
                    .filter(move |k:&u32| *k == key_code)
                    .map_to(());
        let c_left_key_down =
            key_code_pressed(LEFT_KEY_CODE).map_to(true)
                .or_else(&key_code_released(LEFT_KEY_CODE).map_to(false))
                .hold(false);
        let c_right_key_down =
            key_code_pressed(RIGHT_KEY_CODE).map_to(true)
                .or_else(&key_code_released(RIGHT_KEY_CODE).map_to(false))
                .hold(false);
        let c_up_key_down =
            key_code_pressed(UP_KEY_CODE).map_to(true)
                .or_else(&key_code_released(UP_KEY_CODE).map_to(false))
                .hold(false);
        let c_down_key_down =
            key_code_pressed(DOWN_KEY_CODE).map_to(true)
                .or_else(&key_code_released(DOWN_KEY_CODE).map_to(false))
                .hold(false);
        let s_movement: Stream<Vec3<f64>> =
            input_driver.s_tick
                .snapshot4(
                    &c_left_key_down,
                    &c_right_key_down,
                    &c_up_key_down,
                    &c_down_key_down,
                    |_tick: &f64, left_key_down: &bool, right_key_down: &bool, up_key_down: &bool, down_key_down: &bool| {
                        let mut movement = Vec3::<f64>::zero();
                        let mut moved = false;
                        if *left_key_down {
                            movement += Vec3::of(-1.0, 0.0, 0.0);
                        }
                        if *right_key_down {
                            movement += Vec3::of(1.0, 0.0, 0.0);
                        }
                        if *up_key_down {
                            movement += Vec3::of(0.0, 1.0, 0.0);
                        }
                        if *down_key_down {
                            movement += Vec3::of(0.0, -1.0, 0.0);
                        }
                        if movement != Vec3::zero() {
                            return Some(movement);
                        } else {
                            return None;
                        }
                    }
                )
                .filter_option();
        let mut keep_alive = Vec::new();
        let player_set = Rc::new(RefCell::new(HashSet::new()));
        {
            let ecs_manager = ecs_manager.clone();
            let player_set = player_set.clone();
            keep_alive.push(s_movement.listen(
                move |movement: &Vec3<f64>| {
                    let mut ecs_manager = ecs_manager.borrow_mut();
                    let player_set = player_set.borrow();
                    for player in player_set.iter() {
                        let axes_op = ecs_manager.get_component(player, Axes3DComponent::component());
                        if let Some(Axes3DComponent { axes }) = axes_op {
                            let axes2 = Axes3DComponent {
                                axes: Axes3D {
                                    origin: axes.origin + *movement,
                                    orientation: axes.orientation
                                }
                            };
                            ecs_manager.set_component(player, axes2);
                        }
                    }
                }
            ));
        }
        InputSystem {
            ecs_manager,
            player_set: player_set,
            keep_alive
        }
    }
}

impl<ECTX: EcsContext, EVCTX: EcsEventContext<ECTX>> System<ECTX, EVCTX> for InputSystem {

    fn system_name() -> &'static str {
        "InputSystem"
    }

    fn configure(&mut self, event_ctx: &mut EVCTX) {
        {
            let player_set = self.player_set.clone();
            event_ctx.subscribe(
                ComponentSetEvent::ecs_event_type(),
                move |ectx: &mut ECTX, e: &ComponentSetEvent| {
                    if e.component_type_name == PlayerComponent::component().type_name {
                        let mut player_set = player_set.borrow_mut();
                        player_set.insert(e.entity);
                    }
                }
            );
        }
        {
            let player_set = self.player_set.clone();
            event_ctx.subscribe(
                ComponentUnsetEvent::ecs_event_type(),
                move |ectx: &mut ECTX, e: &ComponentUnsetEvent| {
                    if e.component_type_name == PlayerComponent::component().type_name {
                        let mut player_set = player_set.borrow_mut();
                        player_set.remove(&e.entity);
                    }
                }
            );
        }
    }
}