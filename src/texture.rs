use piston_window;
use gfx_device_gl::Resources;
use piston_window::{TextureSettings, Flip};
use std::path::PathBuf;
use super::App;

pub struct Texture {
    texture : piston_window::Texture<Resources>
}

impl Texture {
    pub fn empty(app : &mut App) -> Texture{
        Texture{texture : piston_window::Texture::empty(&mut app.window.factory)
            .expect("Getting empty texture failed. 0_o")}
    }

    pub fn get_raw(&self) -> &piston_window::Texture<Resources> {
        &self.texture
    }

    pub fn set_texture(&mut self, app : &mut App, texture_name : &str) {
        let mut texture_path = PathBuf::from("assets");
        texture_path.push("textures");
        texture_path.push(texture_name);
        let result = piston_window::Texture::from_path(
            &mut app.window.factory,
            texture_path,
            Flip::None,
            &TextureSettings::new()
        );
        match result {
            Ok(texture) => {
                self.texture = texture;
            }
            Err(err) => {
                println!("Unable to load texture, {} Error: {:?}", texture_name, err);
            }
        }
    }
}
