use std::any::Any;
use component::Component;

/// Helper trait for Components, any struct that implements Component and Any automatically
/// implements this trait.  Needed to allow components of a specific type to be queried for on a
/// GameObject.
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
