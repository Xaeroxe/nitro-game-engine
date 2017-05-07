use graphics::Sprite;
use specs::Component;

#[derive(Debug, Copy, Clone)]
pub struct SpriteComponent {
    pub sprite: Sprite,
    pub drawing_layer: i32,
}

impl Component for SpriteComponent {}