use ecs::Component;
use ecs::IsComponent;
use math::Axes3D;

#[derive(Copy, Clone)]
pub struct SpawnPtComponent {
    axes: Axes3D<f64>
}

impl IsComponent for SpawnPtComponent {
    fn component() -> Component<SpawnPtComponent> {
        Component::of("SpawnPtComponent")
    }
}
