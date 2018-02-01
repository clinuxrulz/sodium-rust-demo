use ecs::Component;
use ecs::IsComponent;
use ecs::EcsContext;
use ecs::IsEcsEvent;
use ecs::Entity;
use ecs::System;
use ecs::components::ChildComponent;

use std::collections::HashMap;
use std::option;
use std::boxed::Box;
use std::mem::transmute;
use std::vec::Vec;
use std::any::Any;
use std::ops::Fn;

pub struct EntityManager {
    nextId: u32,
    freeIds: Vec<u32>,
    components: HashMap<String,HashMap<Entity,Box<Any>>>
}

impl EntityManager {

    pub fn new() -> EntityManager {
        EntityManager {
            nextId: 0,
            freeIds: Vec::new(),
            components: HashMap::new()
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        if let Some(id) = self.freeIds.pop() {
            return id;
        }
        let id = self.nextId;
        self.nextId = id + 1;
        return id;
    }

    pub fn destroy_entity(&mut self, entity: &Entity) {
        let keys = self.components.keys().map(|key| key.clone()).collect::<Vec<String>>();
        for key in keys {
            if let Some(map) = self.components.get_mut(&key) {
                map.remove(entity);
            }
        }
    }

    pub fn get_component<T>(&self, entity: &Entity, component: Component<T>) -> Option<T>
    where T: Clone + 'static
    {
        match self.components.get(&component.type_name) {
            Some(map) => {
                match map.get(&entity) {
                    Some(value) => {
                        let value2 = value.downcast_ref::<T>();
                        value2.cloned()
                    },
                    None => None
                }
            },
            None => None
        }
    }

    pub fn set_component<T>(&mut self, entity: &Entity, component: T)
    where T: IsComponent + Clone + 'static
    {
        let component_type_name = T::component().type_name;
        if let Some(map) = self.components.get_mut(&component_type_name) {
            let tmp = Box::new(component) as Box<Any>;
            map.insert(entity.clone(), tmp);
            return;
        }
        let tmp = Box::new(component) as Box<Any>;
        let mut tmp2 = HashMap::new();
        tmp2.insert(entity.clone(), tmp);
        self.components.insert(component_type_name, tmp2);
    }

    pub fn unset_component<T>(&mut self, entity: &Entity, component: Component<T>) {
        let component_type_name = component.type_name;
        if let Some(map) = self.components.get_mut(&component_type_name) {
            let mut wasFound : bool;
            if let Some(tmp) = map.get_mut(&entity) {
                let tmp2 : Box<T> = unsafe { transmute(tmp) };
                wasFound = true;
            } else {
                wasFound = false;
            }
            if wasFound {
                map.remove(&entity);
            }
        }
    }

    pub fn find_children_of(&self, entity: &Entity) -> Vec<Entity> {
        let entity2 = entity.clone();
        let possible_entities = self.entities_with_component(ChildComponent::component());
        let mut result = Vec::new();
        for possible_entity in possible_entities {
            match self.get_component(&possible_entity, ChildComponent::component()) {
                Some(child_component) => {
                    if child_component.parent_entity == entity2 {
                        result.push(possible_entity);
                    }
                }
                None => {}
            }
        }
        return result;
    }

    pub fn entities_with_component<T>(&self, component: Component<T>) -> Vec<Entity> {
        let component_type_name = component.type_name;
        match self.components.get(&component_type_name) {
            Some(map) => map.keys().map(|key| key.clone()).collect(),
            None => Vec::new()
        }
    }
}

impl EcsContext for EntityManager {

    fn transaction<F>(&mut self, doIt: &F)
    where F: Fn(&mut EntityManager)
    {}

    fn get_component<T: Clone + 'static>(&self, entity: &Entity, component: Component<T>) -> Option<T> {
        EntityManager::get_component(self, entity, component)
    }

    fn find_children_of(&self, entity: &Entity) -> Vec<Entity> {
        EntityManager::find_children_of(self, entity)
    }

    fn entities_with_component<T>(&self, component: Component<T>) -> Vec<Entity> {
        EntityManager::entities_with_component(self, component)
    }

    fn create_entity(&mut self) -> Entity {
        EntityManager::create_entity(self)
    }

    fn destroy_entity(&mut self, entity: &Entity) {
        EntityManager::destroy_entity(self, entity)
    }

    fn set_component<T: IsComponent + Clone + 'static>(&mut self, entity: &Entity, component: T) {
        EntityManager::set_component(self, entity, component)
    }

    fn unset_component<T>(&mut self, entity: &Entity, component: Component<T>) {
        EntityManager::unset_component(self, entity, component)
    }
}
