pub mod texture;
pub mod sprite_sheet;

use self::texture::Texture;
use self::sprite_sheet::SpriteSheet;
pub enum Sprite {
    Texture(Texture),
    SpriteSheet(SpriteSheet),
}
