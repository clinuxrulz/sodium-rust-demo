use ecs::Component;
use ecs::IsComponent;
use math::Axes3D;

#[derive(Copy, Clone)]
pub struct Axes3DComponent {
    pub axes: Axes3D<f64>
}

impl IsComponent for Axes3DComponent {
    fn component() -> Component<Axes3DComponent> {
        Component::of("Axes3DComponent")
    }
}
