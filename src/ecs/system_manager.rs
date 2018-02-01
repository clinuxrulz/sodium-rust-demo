use ecs::Component;
use ecs::IsComponent;
use ecs::EcsContext;
use ecs::EcsEventType;
use ecs::IsEcsEvent;
use ecs::EcsEventContext;
use ecs::Entity;
use ecs::EntityManager;
use ecs::System;
use ecs::components::ChildComponent;

use std::collections::HashMap;
use std::option;
use std::boxed;
use std::boxed::Box;
use std::mem::transmute;
use std::vec::Vec;
use std::any::Any;

struct SystemEntry<ECTX: EcsContext> {
    system_name: String,
    handlers: EventHandlers<ECTX>
}

pub struct SystemManager<ECTX: EcsContext> {
    systems: Vec<SystemEntry<ECTX>>
}

pub struct EventHandlers<ECTX: EcsContext> {
    handlers: HashMap<String,Box<FnMut(&mut ECTX,&Any)>>
}

impl<ECTX: EcsContext> EventHandlers<ECTX> {
    fn new() -> EventHandlers<ECTX> {
        EventHandlers {
            handlers: HashMap::new()
        }
    }

    fn lookup_handler(&mut self, event_name: &String) -> Option<&mut Box<FnMut(&mut ECTX,&Any)>> {
        self.handlers.get_mut(event_name)
    }
}

impl<ECTX: EcsContext> EcsEventContext<ECTX> for EventHandlers<ECTX> {
    fn subscribe<E: IsEcsEvent + 'static, F: FnMut(&mut ECTX, &E) + 'static>(&mut self, event_type: EcsEventType<E>, mut handler: F) {
        let event_name = event_type.type_name;
        self.handlers.insert(
            event_name.clone(),
            Box::new(
                move |ectx, event| {
                    let event_op = event.downcast_ref();
                    if let Some(event2) = event_op {
                        handler(ectx, event2);
                    }
                }
            )
        );
    }
}

impl<ECTX: EcsContext> SystemManager<ECTX> {

    pub fn new() -> SystemManager<ECTX> {
        SystemManager {
            systems: Vec::new()
        }
    }

    pub fn add_system<SYSTEM>(&mut self, mut system: SYSTEM)
    where
    SYSTEM: System<ECTX,EventHandlers<ECTX>>
    {
        let mut event_ctx = EventHandlers::new();
        system.configure(&mut event_ctx);
        self.systems.push(
            SystemEntry {
                system_name: String::from(SYSTEM::system_name()),
                handlers: event_ctx
            }
        );
    }

    pub fn fire_event<E: IsEcsEvent + 'static>(&mut self, ecs_ctx: &mut ECTX, event: &E) {
        let event_name = E::ecs_event_type().type_name;
        let mut handlers_for_event = Vec::new();
        {
            for system in self.systems.iter_mut() {
                match system.handlers.lookup_handler(&event_name) {
                    Some(handler) => handlers_for_event.push(handler),
                    None => ()
                }
            }
        }
        for handler in handlers_for_event.iter_mut() {
            handler(ecs_ctx, event);
        }
    }

    pub fn fire_boxed_event(&mut self, ecs_ctx: &mut ECTX, event: &(String,Box<Any>)) {
        let &(ref event_name,ref event_value) = event;
        let mut handlers_for_event = Vec::new();
        {
            for system in self.systems.iter_mut() {
                match system.handlers.lookup_handler(&event_name) {
                    Some(handler) => handlers_for_event.push(handler),
                    None => ()
                }
            }
        }
        for handler in handlers_for_event.iter_mut() {
            handler(ecs_ctx, event_value.as_ref());
        }
    }
}
