use game_object::GameObject;

pub trait UpdateComponent {
    fn update(&mut self, game_object : &mut GameObject, delta_time : f64);
}
