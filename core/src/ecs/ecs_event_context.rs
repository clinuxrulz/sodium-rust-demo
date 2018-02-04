use ecs::EcsContext;
use ecs::IsEcsEvent;
use ecs::EcsEventType;

use std::ops::FnMut;

pub trait EcsEventContext<ECTX: EcsContext> {
    fn subscribe<E: IsEcsEvent + 'static, F: FnMut(&mut ECTX, &E) + 'static>(&mut self, event_type: EcsEventType<E>, handler: F);
}
