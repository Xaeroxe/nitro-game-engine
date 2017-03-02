use app::App;
use game_object::GameObject;
use std::any::Any;

pub trait Component {
    fn receive_message(&mut self, app: &mut App, game_object: &mut GameObject, message: &Message);
}

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

pub enum Message {
    Start { key: i32 },
    Update { delta_time: f32 },
    UserMessage(Box<Any>),
}
