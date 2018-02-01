use ecs::EcsEventType;
use ecs::Entity;
use ecs::IsEcsEvent;

pub struct ComponentSetEvent {
    pub entity: Entity,
    pub component_type_name: String
}

impl IsEcsEvent for ComponentSetEvent {
    fn ecs_event_type() -> EcsEventType<Self> {
        EcsEventType::new("ComponentSetEvent")
    }
}
