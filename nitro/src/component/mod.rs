use app::App;
use game_object::GameObject;
use std::any::Any;

pub trait Component: AsRef<Any> + AsMut<Any> {
    fn receive_message(&mut self, app: &mut App, game_object: &mut GameObject, message: &Message);
}

pub enum Message {
    Start { key: i32 },
    Update { delta_time: f64 },
    UserMessage(Box<Any>),
}
