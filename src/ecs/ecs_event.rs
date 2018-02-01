use std::marker::PhantomData;

pub struct EcsEventType<E> {
    pub type_name: String,
    phantom_data: PhantomData<E>
}

impl<E> EcsEventType<E> {
    pub fn new(type_name: &'static str) -> EcsEventType<E> {
        EcsEventType {
            type_name: String::from(type_name),
            phantom_data: PhantomData
        }
    }
}

pub trait IsEcsEvent {
    fn ecs_event_type() -> EcsEventType<Self>
    where Self: Sized;
}
