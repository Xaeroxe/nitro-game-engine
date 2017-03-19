use std::any::Any;
use component::Component;

pub trait ComponentAny: Component + Any {
    fn as_any(&self) -> &Any;
    fn as_any_mut(&mut self) -> &mut Any;
}

impl<T> ComponentAny for T
    where T: Component + Any
{
    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }
}

