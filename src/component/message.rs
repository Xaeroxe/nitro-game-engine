use std::any::Any;

pub enum Message {
    /// Sent to Components the instant they are attached to a GameObject.
    ///
    /// This will be the Component's only opportunity to know its own key.  If you need this please
    /// store the key value.
    Start { key: i32 },

    /// Sent to Components every frame, delta_time is the time elapsed since last frame in seconds.
    Update { delta_time: f32 },

    /// Sent to all components on a GameObject prior to the GameObject being destroyed.
    OnDestroy,

    /// Sent when this component is removed from the GameObject.
    OnDetach,

    /// Whatever you want it to be!  You can send and receive this message type as much as you
    /// like, Nitro will never send these.
    UserMessage(Box<Any>),
}
