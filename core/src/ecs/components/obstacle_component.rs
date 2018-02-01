use ecs::Component;
use ecs::IsComponent;

#[derive(Copy, Clone)]
pub struct ObstacleComponent {}

impl IsComponent for ObstacleComponent {
    fn component() -> Component<ObstacleComponent> {
        Component::of("ObstacleComponent")
    }
}
