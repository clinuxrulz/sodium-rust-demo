use ecs::Component;
use ecs::IsComponent;

#[derive(Copy, Clone)]
pub struct PlayerComponent {}

impl IsComponent for PlayerComponent {
    fn component() -> Component<PlayerComponent> {
        Component::of("PlayerComponent")
    }
}
