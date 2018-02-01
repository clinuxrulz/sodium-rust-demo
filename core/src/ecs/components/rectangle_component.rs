use ecs::Component;
use ecs::IsComponent;

#[derive(Copy, Clone)]
pub struct RectangleComponent {
    pub width: f64,
    pub height: f64
}

impl IsComponent for RectangleComponent {
    fn component() -> Component<RectangleComponent> {
        Component::of("RectangleComponent")
    }
}
