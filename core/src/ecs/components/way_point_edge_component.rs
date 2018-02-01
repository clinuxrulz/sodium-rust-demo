use ecs::Component;
use ecs::Entity;
use ecs::IsComponent;

#[derive(Copy, Clone)]
pub struct WayPointEdgeComponent {
    pub node1: Entity,
    pub node2: Entity
}

impl IsComponent for WayPointEdgeComponent {
    fn component() -> Component<WayPointEdgeComponent> {
        Component::of("WayPointEdgeComponent")
    }
}
