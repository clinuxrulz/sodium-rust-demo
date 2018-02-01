use ecs::Component;
use ecs::IsComponent;
use math::Line3D;

#[derive(Copy, Clone)]
pub struct LineComponent {
    pub line: Line3D<f64>
}

impl IsComponent for LineComponent {
    fn component() -> Component<LineComponent> {
        Component::of("LineComponent")
    }
}
