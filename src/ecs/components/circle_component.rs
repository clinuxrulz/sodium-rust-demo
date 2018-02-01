use ecs::Component;
use ecs::IsComponent;

#[derive(Copy, Clone)]
pub struct CircleComponent {
    pub radius: f64
}

impl IsComponent for CircleComponent {
    fn component() -> Component<CircleComponent> {
        Component::of("CircleComponent")
    }
}
