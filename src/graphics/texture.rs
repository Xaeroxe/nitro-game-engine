use sdl2::render::Texture as SdlTexture;
use std::sync::Arc;

/// A static handle to a texture whose data is managed by Nitro.
pub struct Texture {
    width: u32,
    height: u32,
    texture: Arc<SdlTexture>,
    pub flip_vertical: bool,
    pub flip_horizontal: bool,
}

impl Texture {
    pub(crate) fn new(texture: Arc<SdlTexture>) -> Texture {
        let query = texture.query();
        Texture {
            width: query.width,
            height: query.height,
            texture: texture,
            flip_vertical: false,
            flip_horizontal: false,
        }
    }

    pub(crate) fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub(crate) fn get_raw(&self) -> &Arc<SdlTexture> {
        &self.texture
    }
}

