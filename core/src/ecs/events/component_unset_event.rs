use ecs::EcsEventType;
use ecs::Entity;
use ecs::IsEcsEvent;

pub struct ComponentUnsetEvent {
    pub entity: Entity,
    pub component_type_name: String
}

impl IsEcsEvent for ComponentUnsetEvent {
    fn ecs_event_type() -> EcsEventType<Self> {
        EcsEventType::new("ComponentUnsetEvent")
    }
}
