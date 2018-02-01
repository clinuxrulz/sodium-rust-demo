use ecs::Entity;
use ecs::Component;
use ecs::IsComponent;

use std::vec::Vec;
use std::option;
use std::ops::Fn;

pub trait EcsContext {

    fn transaction<F>(&mut self, doIt: &F)
    where F: Fn(&mut Self);

    fn get_component<T: Clone + 'static>(&self, entity: &Entity, component: Component<T>) -> Option<T>;

    fn with_component<T: Clone + 'static, R, F: FnMut(&Self,&T)->R >(&self, entity: &Entity, component: Component<T>, k: &mut F) -> Option<R> {
        let comp_op = self.get_component(entity, component);
        match comp_op {
            Some(comp) => Some(k(self, &comp)),
            None => None
        }
    }

    fn with_component_mut<T: Clone + 'static, R, F: FnMut(&Self,&T)->R >(&mut self, entity: &Entity, component: Component<T>, k: &mut F) -> Option<R> {
        let comp_op = self.get_component(entity, component);
        match comp_op {
            Some(comp) => Some(k(self, &comp)),
            None => None
        }
    }

    fn find_children_of(&self, entity: &Entity) -> Vec<Entity>;

    fn entities_with_component<T>(&self, component: Component<T>) -> Vec<Entity>;

    fn create_entity(&mut self) -> Entity;

    fn destroy_entity(&mut self, entity: &Entity);

    fn set_component<T: IsComponent + Clone + 'static>(&mut self, entity: &Entity, component: T);

    fn unset_component<T>(&mut self, entity: &Entity, component: Component<T>);
}
