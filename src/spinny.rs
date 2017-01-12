use nitro::App;
use nitro::component::Component;
use nitro::component::Message;
use nitro::GameObject;
use axes::AxisId;
use actions::ActionId;
use std::any::Any;

pub struct Spinny {

}

impl Component for Spinny {
    fn receive_message(&mut self, app: &mut App, game_object: &mut GameObject, message: &Message) {
        match *message {
            Message::Update { delta_time } => {
                if let Some(horizontal) = app.get_axis_value(AxisId::Horizontal as i32) {
                    *game_object.transform.mut_position() += 100.0 * delta_time * horizontal *
                                                             game_object.transform.right_vec2();
                }
                if let Some(vertical) = app.get_axis_value(AxisId::Vertical as i32) {
                    *game_object.transform.mut_position() += 100.0 * delta_time * vertical *
                                                             game_object.transform.forward_vec2();
                }
                if let Some(rotation) = app.get_axis_value(AxisId::Rotation as i32) {
                    *game_object.transform.mut_rotation() += 1.0 * delta_time * rotation;
                }
                if let Some(true) = app.action_released(ActionId::Blink as i32) {
                    *game_object.transform.mut_x() += 50.0;
                }
            }
            _ => {}
        }
    }

    fn as_any(&self) -> &Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut Any {
        self
    }
}
