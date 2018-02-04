use ecs::Component;
use ecs::EcsContext;
use ecs::Entity;
use ecs::EntityManager;
use ecs::IsComponent;
use ecs::IsEcsEvent;
use ecs::SystemManager;
use ecs::System;
use ecs::EventHandlers;
use ecs::events::ComponentSetEvent;

use std::ops::Fn;
use std::vec::Vec;
use std::any::Any;

pub struct EcsManager<ENV> {
    entity_manager: EntityManager,
    system_manager: SystemManager<ENV,EntityManager>,
    transaction_depth: u32,
    backed_up_events: Vec<(String,Box<Any + 'static>)>
}

impl<ENV> EcsManager<ENV> {

    pub fn new() -> EcsManager<ENV> {
        EcsManager {
            entity_manager: EntityManager::new(),
            system_manager: SystemManager::new(),
            transaction_depth: 0,
            backed_up_events: Vec::new()
        }
    }

    pub fn add_system<SYSTEM>(&mut self, system: SYSTEM)
    where
    SYSTEM: System<ENV,EntityManager,EventHandlers<ENV,EntityManager>>
    {
        self.system_manager.add_system(system);
    }

    pub fn fire_event<E: IsEcsEvent + Copy + 'static>(&mut self, env: &mut ENV, event: &E) {
        let tdepth = self.transaction_depth.clone();
        if tdepth == 0 {
            self.system_manager.fire_event(env, &mut self.entity_manager, event);
        } else {
            self.backed_up_events.push((E::ecs_event_type().type_name,Box::new(event.clone())));
        }
    }
}

pub struct EcsManagerWithENV<'r,ENV:'r> {
    pub env: &'r mut ENV,
    pub ecs_manager: EcsManager<ENV>
}

impl<'r,ENV> EcsContext for EcsManagerWithENV<'r,ENV> {

    fn transaction<F>(&mut self, do_it: &F)
    where F: Fn(&mut EcsManagerWithENV<'r,ENV>)
    {
        self.ecs_manager.transaction_depth = self.ecs_manager.transaction_depth + 1;
        do_it(self);
        self.ecs_manager.transaction_depth = self.ecs_manager.transaction_depth - 1;
        let tdepth = self.ecs_manager.transaction_depth;
        if tdepth == 0 {
            let entity_manager = &mut self.ecs_manager.entity_manager;
            let system_manager = &mut self.ecs_manager.system_manager;
            let backed_up_events = &self.ecs_manager.backed_up_events;
            for event in backed_up_events {
                system_manager.fire_boxed_event(self.env, entity_manager, event);
            }
        }
    }

    fn get_component<T: Clone + 'static>(&self, entity: &Entity, component: Component<T>) -> Option<T> {
        self.ecs_manager.entity_manager.get_component(entity, component)
    }

    fn find_children_of(&self, entity: &Entity) -> Vec<Entity> {
        self.ecs_manager.entity_manager.find_children_of(entity)
    }

    fn entities_with_component<T>(&self, component: Component<T>) -> Vec<Entity> {
        self.ecs_manager.entity_manager.entities_with_component(component)
    }

    fn create_entity(&mut self) -> Entity {
        self.ecs_manager.entity_manager.create_entity()
    }

    fn destroy_entity(&mut self, entity: &Entity) {
        self.ecs_manager.entity_manager.destroy_entity(entity)
    }

    fn set_component<T: IsComponent + Clone + 'static>(&mut self, entity: &Entity, component: T) {
        self.ecs_manager.entity_manager.set_component(entity, component);
        self.ecs_manager.system_manager.fire_event(
            self.env,
            &mut self.ecs_manager.entity_manager,
            &ComponentSetEvent {
                entity: entity.clone(),
                component_type_name: T::component().type_name
            }
        );
    }

    fn unset_component<T>(&mut self, entity: &Entity, component: Component<T>) {
        self.ecs_manager.entity_manager.unset_component(entity, component);
    }
}
