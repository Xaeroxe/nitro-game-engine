use game_object::GameObject;
use std::any::Any;

pub trait Component {
    fn receive_message(&mut self, game_object : &mut GameObject, message : &Message);
}

pub enum Message {
    Update{delta_time : f64},
    UserMessage(Box<Any>)
}
