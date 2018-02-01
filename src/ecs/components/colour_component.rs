use ecs::Component;
use ecs::IsComponent;
use math::RGBA;

#[derive(Copy, Clone)]
pub struct ColourComponent {
    pub rgba: RGBA<u8>
}

impl IsComponent for ColourComponent {
    fn component() -> Component<ColourComponent> {
        Component::of("ColourComponent")
    }
}
