use Canvas;
use app::App;
use game_object::GameObject;
use component::Message;

pub trait Component {
    fn receive_message(&mut self, app: &mut App, game_object: &mut GameObject, message: &Message);
    fn render_gui(&self, canvas: &mut Canvas, game_object: &GameObject);
}
