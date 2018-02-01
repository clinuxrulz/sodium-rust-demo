use ecs::Component;
use ecs::IsComponent;

#[derive(Copy, Clone)]
pub struct BoxComponent {
    pub length: f64,
    pub width: f64,
    pub height: f64
}

impl IsComponent for BoxComponent {
    fn component() -> Component<BoxComponent> {
        Component::of("BoxComponent")
    }
}
