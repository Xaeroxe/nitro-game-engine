use piston_window::PistonWindow;
use piston_window::Texture;
use piston_window::TextureSettings;
use piston_window::Flip;
use graphics::math::Matrix2d;
use std::path::PathBuf;
use gfx_device_gl::Resources;

pub struct GameObject {
    pub x : f64,
    pub y : f64,
    pub rot : f64,
    pub texture : Texture<Resources>,
}

impl GameObject {
    pub fn new(window : &mut PistonWindow) -> GameObject {
        GameObject{x:0.0,y:0.0,rot:0.0,texture:Texture::empty(&mut window.factory).expect("Bro it's an empty texture.  How could this fail?")}
    }

    pub fn set_texture(&mut self, window : &mut PistonWindow, texture_name : &str) {
        let mut texture_path = PathBuf::from("assets");
        texture_path.push("textures");
        texture_path.push(texture_name);
        let result = Texture::from_path(&mut window.factory, texture_path, Flip::None, &TextureSettings::new());
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
