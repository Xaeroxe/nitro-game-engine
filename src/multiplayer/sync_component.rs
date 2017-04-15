use component::Component;
use app::App;
use game_object::GameObject;
use component::Message;
use graphics::Canvas;

pub struct SyncComponent {

}

impl Component for SyncComponent {
    fn receive_message(&mut self, app: &mut App, game_object: &mut GameObject, message: &Message) {

    }

    fn render_gui(&self, canvas: &mut Canvas, game_object: &GameObject) {
    
    }
}
