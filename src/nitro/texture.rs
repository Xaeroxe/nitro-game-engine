use piston_window;
use gfx_device_gl::Resources;
use nitro::app::App;

pub struct Texture {
    texture : piston_window::Texture<Resources>
}

impl Texture {
    pub fn empty(app : &mut App) -> Texture{
        Texture{texture : app.empty_raw_texture()}
    }

    pub fn get_raw(&self) -> &piston_window::Texture<Resources> {
        &self.texture
    }

    pub fn set_raw_texture(&mut self, texture : piston_window::Texture<Resources>) {
        self.texture = texture;
    }
}
