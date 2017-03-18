use graphics::Texture;
use graphics::SpriteSheet;

pub enum Sprite {
    Texture(Texture),
    SpriteSheet(SpriteSheet),
}
