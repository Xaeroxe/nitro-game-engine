pub mod texture;

use self::texture::Texture;
pub enum Sprite {
    Texture(Texture),
}
