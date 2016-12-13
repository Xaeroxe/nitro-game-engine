use game_object::GameObject;

pub trait Component {
    fn receive_message(&mut self, game_object : &mut GameObject, message : &Message);
}

pub enum Message {
    Update{delta_time : f64},
}
