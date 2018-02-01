use ecs::Component;
use ecs::IsComponent;
use math::Vec3;

#[derive(Copy, Clone)]
pub struct WayPointNodeComponent {
    pub location: Vec3<f64>
}

impl IsComponent for WayPointNodeComponent {
    fn component() -> Component<WayPointNodeComponent> {
        Component::of("WayPointNodeComponent")
    }
}
