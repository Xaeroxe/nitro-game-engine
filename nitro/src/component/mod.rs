use app::App;
use game_object::GameObject;
use std::any::Any;

pub trait Component {
    fn receive_message(&mut self, app: &mut App, game_object: &mut GameObject, message: &Message);
    fn as_any(&self) -> &Any;
    fn as_any_mut(&mut self) -> &mut Any;
}

pub enum Message {
    Update { delta_time: f64 },
    UserMessage(Box<Any>),
}
