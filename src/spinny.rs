use nitro::App;
use nitro::Canvas;
use nitro::component::Component;
use nitro::component::Message;
use nitro::GameObject;
use axes::AxisId;
use actions::ActionId;

pub struct Spinny {}


impl Component for Spinny {
    fn receive_message(&mut self, app: &mut App, game_object: &mut GameObject, message: &Message) {
        match *message {
            Message::Start { .. } => {
                app.play_sound("example.ogg", 1.0).expect("Sound failed to play.");
            }

            Message::Update { delta_time } => {
                if let Some(horizontal) = app.input.get_axis_value(AxisId::Horizontal as i32) {
                    *game_object.transform.mut_position() += 100.0 * delta_time * horizontal *
                                                             game_object.transform.right();
                }
                if let Some(vertical) = app.input.get_axis_value(AxisId::Vertical as i32) {
                    *game_object.transform.mut_position() += 100.0 * delta_time * vertical *
                                                             game_object.transform.forward();
                }
                if let Some(rotation) = app.input.get_axis_value(AxisId::Rotation as i32) {
                    *game_object.transform.mut_rotation() += 1.0 * delta_time * rotation;
                }
                if let Some(true) = app.input.action_released(ActionId::Blink as i32) {
                    *game_object.transform.mut_x() += 50.0;
                }
            }
            _ => {}
        }
    }
    fn render_gui(&self, canvas: &mut Canvas, game_object: &GameObject) {}
}
