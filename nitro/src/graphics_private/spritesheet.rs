use sdl2::render::Texture as SdlTexture;
use std::sync::Arc;
use Rect;

pub struct SpriteSheet {
    pub animations: Vec<Vec<SpriteSheetFrame>>,
    texture: Arc<SdlTexture>,
}

pub struct SpriteSheetFrame {
    pub frame_rect: Rect,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
}
