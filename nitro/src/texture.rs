use sdl2::render::Texture as SdlTexture;
use app::App;

pub struct Texture {
    width: u32,
    height: u32,
    texture: Option<SdlTexture>,
}

impl Texture {
    pub fn new() -> Texture {
        Texture {
            width: 0,
            height: 0,
            texture: None,
        }
    }
}

pub fn size(nitro_texture: &Texture) -> (u32, u32) {
    (nitro_texture.width, nitro_texture.height)
}

pub fn get_raw(nitro_texture: &Texture) -> &Option<SdlTexture> {
    &nitro_texture.texture
}

pub fn set_raw(nitro_texture: &mut Texture, texture: SdlTexture) {
    let query = texture.query();
    nitro_texture.width = query.width;
    nitro_texture.height = query.height;
    nitro_texture.texture = Some(texture);
}
