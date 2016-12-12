use piston_window::PistonWindow;
use piston_window::Texture;
use piston_window::TextureSettings;
use piston_window::Flip;
use std::path::PathBuf;
use gfx_device_gl::Resources;
use update_component::UpdateComponent;
use std::mem;

pub struct GameObject {
    pub x : f64,
    pub y : f64,
    pub rot : f64,
    pub texture : Texture<Resources>,
    update_components : Vec<Box<UpdateComponent>>, //These have not had update run this frame (yet).
    updated_components: Vec<Box<UpdateComponent>>, //These have had update run this frame.
}

impl GameObject {
    pub fn new(window : &mut PistonWindow) -> GameObject {
        GameObject{
            x : 0.0,
            y : 0.0,
            rot : 0.0,
            update_components : Vec::new(),
            updated_components : Vec::new(),
            texture : Texture::empty(&mut window.factory)
                .expect("Bro it's an empty texture.  How could this fail?"),
        }
    }

    pub fn update(&mut self, delta_time : f64) {
        let mut pop_result = self.update_components.pop();
        while let Some(mut component) = pop_result {
            component.update(self, delta_time);
            self.updated_components.push(component);
            pop_result = self.update_components.pop();
        }
        assert_eq!(self.update_components.len(), 0);
        mem::swap(&mut self.update_components, &mut self.updated_components);
    }

    pub fn add_update_component(&mut self, component : Box<UpdateComponent>)
    {
        self.update_components.push(component);
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
