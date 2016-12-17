use nitro::App;
use nitro::component::Component;
use nitro::component::Message;
use nitro::GameObject;
use axes::AxisId;

pub struct Spinny {

}

impl Component for Spinny {
    fn receive_message(&mut self, app : &mut App, game_object : &mut GameObject, message : &Message) {
        match *message {
            Message::Update{delta_time} => {
                //game_object.transform.add_rotation(1.0 * delta_time);
                if let Some(horizontal) = app.get_axis_value(AxisId::Horizontal as i32) {
                    *game_object.transform.x() += 100.0 * delta_time * horizontal;
                }
                if let Some(vertical) = app.get_axis_value(AxisId::Vertical as i32) {
                    *game_object.transform.y() += 100.0 * delta_time * vertical;
                }
            }
            _ => {}
        }
    }
}
