use ecs::Component;
use ecs::IsComponent;
use ecs::Entity;

#[derive(Copy, Clone)]
pub struct ChildComponent {
    pub parent_entity: Entity
}

impl IsComponent for ChildComponent {
    fn component() -> Component<ChildComponent> {
        Component::of("ChildComponent")
    }
}
