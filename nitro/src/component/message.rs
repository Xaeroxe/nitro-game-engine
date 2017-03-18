use std::any::Any;

pub enum Message {
    Start { key: i32 },
    Update { delta_time: f32 },
    OnDestroy,
    OnDetach,
    UserMessage(Box<Any>),
}
