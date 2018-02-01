use ecs::Component;
use ecs::IsComponent;

#[derive(Copy, Clone)]
pub struct ZombieComponent {}

impl IsComponent for ZombieComponent {
    fn component() -> Component<ZombieComponent> {
        Component::of("ZombieComponent")
    }
}
