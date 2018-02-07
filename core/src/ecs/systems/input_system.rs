use drivers::InputDriver;
use ecs::EcsContext;
use ecs::EcsEventContext;
use ecs::EcsManager;
use ecs::Entity;
use ecs::IsComponent;
use ecs::IsEcsEvent;
use ecs::System;
use ecs::components::PlayerComponent;
use ecs::events::ComponentSetEvent;
use ecs::events::ComponentUnsetEvent;

use sodium::IsStream;
use sodium::Listener;

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

pub struct InputSystem {
    ecs_manager: Rc<RefCell<EcsManager>>,
    player_set: Rc<RefCell<HashSet<Entity>>>,
    keep_alive: Vec<Listener>
}

impl InputSystem {
    fn new(input_driver: Rc<RefCell<InputDriver>>, ecs_manager: Rc<RefCell<EcsManager>>) -> InputSystem {
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
        let keep_alive = Vec::new();
        InputSystem {
            ecs_manager,
            player_set: Rc::new(RefCell::new(HashSet::new())),
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