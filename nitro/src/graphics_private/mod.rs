pub mod texture;
pub mod spritesheet;

use self::texture::Texture;
use self::spritesheet::SpriteSheet;
pub enum Sprite {
    Texture(Texture),
    SpriteSheet(SpriteSheet),
}
