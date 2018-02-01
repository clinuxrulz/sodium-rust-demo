use ecs::EcsEventType;
use ecs::Entity;
use ecs::IsEcsEvent;

pub struct EntityCreatedEvent {
    pub entity: Entity
}

impl IsEcsEvent for EntityCreatedEvent {
    fn ecs_event_type() -> EcsEventType<Self> {
        EcsEventType::new("EntityCreatedEvent")
    }
}
