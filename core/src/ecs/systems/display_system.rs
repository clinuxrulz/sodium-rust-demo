use ecs::EcsContext;
use ecs::EcsEventContext;
use ecs::Entity;
use ecs::IsComponent;
use ecs::IsEcsEvent;
use ecs::System;
use ecs::components::Axes3DComponent;
use ecs::components::CircleComponent;
use ecs::events::ComponentSetEvent;
use ecs::events::ComponentUnsetEvent;
use ecs::util::ecs_get_axes3d;
use drivers::DisplayDriver;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct DisplaySystem {
    display_driver: Rc<RefCell<DisplayDriver>>,
    entity_circle_map: Rc<RefCell<HashMap<Entity,u32>>>
}

impl DisplaySystem {
    pub fn new(display_driver: Rc<RefCell<DisplayDriver>>) -> DisplaySystem {
        DisplaySystem {
            display_driver,
            entity_circle_map: Rc::new(RefCell::new(HashMap::new()))
        }
    }
}

impl<ECTX: EcsContext, EVCTX: EcsEventContext<ECTX>> System<ECTX, EVCTX> for DisplaySystem {

    fn system_name() -> &'static str {
        "DisplaySystem"
    }

    fn configure(&mut self, event_ctx: &mut EVCTX) {
        {
            let display_driver = Rc::clone(&self.display_driver);
            let entity_circle_map = Rc::clone(&self.entity_circle_map);
            event_ctx.subscribe(
                ComponentSetEvent::ecs_event_type(),
                move |ectx: &mut ECTX, e: &ComponentSetEvent| {
                    if e.component_type_name == CircleComponent::component().type_name {
                        let mut display_driver = display_driver.borrow_mut();
                        let mut entity_circle_map = entity_circle_map.borrow_mut();
                        let axes = ecs_get_axes3d(ectx, &e.entity);
                        let circle = display_driver.add_circle(axes.origin.x, axes.origin.y, 10.0);
                        entity_circle_map.insert(e.entity, circle);
                    } else if e.component_type_name == Axes3DComponent::component().type_name {
                        if ectx.get_component(&e.entity, CircleComponent::component()).is_some() {
                            let mut display_driver = display_driver.borrow_mut();
                            let entity_circle_map = entity_circle_map.borrow();
                            let axes = ecs_get_axes3d(ectx, &e.entity);
                            if let Some(circle) = entity_circle_map.get(&e.entity) {
                                display_driver.move_circle(*circle, axes.origin.x, axes.origin.y);
                            }
                        }
                    }
                }
            );
        }
        {
            let display_driver = Rc::clone(&self.display_driver);
            let entity_circle_map = Rc::clone(&self.entity_circle_map);
            event_ctx.subscribe(
                ComponentUnsetEvent::ecs_event_type(),
                move |_ectx: &mut ECTX, e: &ComponentUnsetEvent| {
                    if e.component_type_name == CircleComponent::component().type_name {
                        let mut display_driver = display_driver.borrow_mut();
                        let mut entity_circle_map = entity_circle_map.borrow_mut();
                        if let Some(circle) = entity_circle_map.remove(&e.entity) {
                            display_driver.remove(circle);
                        }
                    }
                }
            )
        }
    }
}
