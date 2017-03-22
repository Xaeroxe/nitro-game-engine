use sdl2::render::Texture as SdlTexture;
use std::sync::Arc;

pub struct Texture {
    width: u32,
    height: u32,
    texture: Arc<SdlTexture>,
    pub flip_vertical: bool,
    pub flip_horizontal: bool,
}

pub fn new(texture: Arc<SdlTexture>) -> Texture {
    let query = texture.query();
    Texture {
        width: query.width,
        height: query.height,
        texture: texture,
        flip_vertical: false,
        flip_horizontal: false,
    }
}

pub fn size(nitro_texture: &Texture) -> (u32, u32) {
    (nitro_texture.width, nitro_texture.height)
}

pub fn get_raw(nitro_texture: &Texture) -> &Arc<SdlTexture> {
    &nitro_texture.texture
}
