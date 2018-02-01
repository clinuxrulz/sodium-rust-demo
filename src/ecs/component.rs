use std::marker::PhantomData;

pub struct Component<A> {
    pub type_name: String,
    phantom_type: PhantomData<A>
}

impl<A> Component<A> {
    pub fn of(type_name: &'static str) -> Component<A> {
        Component {
            type_name: String::from(type_name),
            phantom_type: PhantomData
        }
    }
}

pub trait IsComponent where Self: Sized {
    fn component() -> Component<Self>;
}
