use app::App;
use game_object::GameObject;
use std::any::Any;
use std::cell::RefCell;
use audio::Dj;

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

pub enum Message<'a> {
    Start { key: i32 },
    Update { delta_time: f32 },
    DjIdle { dj: RefCell<&'a mut Dj> },
    UserMessage(Box<Any>),
}
