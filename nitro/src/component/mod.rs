

use app::App;
use game_object::GameObject;
use std::any::Any;

pub trait Component: AsRef<Any> + AsMut<Any> + 'static {
    fn receive_message(&mut self, app: &mut App, game_object: &mut GameObject, message: &Message);
}

#[macro_export]
macro_rules! to_any {
    ($t:ty) => {
        impl AsRef<Any> for $t {
            fn as_ref(&self) -> &Any {
                self
            }
        }

        impl AsMut<Any> for $t {
            fn as_mut(&mut self) -> &mut Any {
                self
            }
        }
    }
}

pub enum Message {
    Update { delta_time: f64 },
    UserMessage(Box<Any>),
}
