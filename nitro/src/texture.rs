use piston_window;
use gfx_device_gl::Resources;
use app::App;

pub struct Texture {
    texture : piston_window::Texture<Resources>
}

impl Texture {
    pub fn empty(app : &mut App) -> Texture{
        Texture{texture : app.empty_raw_texture()}
    }
}

pub fn get_raw(nitro_texture : &Texture) -> &piston_window::Texture<Resources> {
    &nitro_texture.texture
}

pub fn set_raw_texture(nitro_texture : &mut Texture, texture : piston_window::Texture<Resources>) {
    nitro_texture.texture = texture;
}
