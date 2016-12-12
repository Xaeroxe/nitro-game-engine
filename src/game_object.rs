use piston_window::PistonWindow;
use piston_window::Texture;
use piston_window::TextureSettings;
use piston_window::Flip;
use graphics::math::Matrix2d;
use std::path::PathBuf;
use gfx_device_gl::Resources;
use update_component::UpdateComponent;
use std::mem;

pub struct GameObject {
    transform : Matrix2d,
    pub texture : Texture<Resources>,
    update_components : Vec<Box<UpdateComponent>>, //These have not had update run this frame (yet).
    updated_components: Vec<Box<UpdateComponent>>, //These have had update run this frame.
}

impl GameObject {
    pub fn new(window : &mut PistonWindow) -> GameObject {
        GameObject{
            transform : [[1.0, 0.0, 0.0],[0.0, 1.0, 0.0]], // Identity format for Matrix2d. Reverse engineered from Piston.
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

    pub fn add_update_component(&mut self, component : Box<UpdateComponent>) {
        self.update_components.push(component);
    }

    pub fn get_draw_matrix(&self) -> Matrix2d {
        self.transform
    }

    pub fn set_x(&mut self, x : f64) {
        self.transform[0][2] = x;
    }

    pub fn get_x(&self) -> f64 {
        self.transform[0][2]
    }

    pub fn add_x(&mut self, delta_x : f64) {
        let x = self.get_x();
        self.set_x(x + delta_x);
    }

    pub fn set_y(&mut self, y : f64) {
        self.transform[1][2] = y;
    }

    pub fn get_y(&self) -> f64 {
        self.transform[1][2]
    }

    pub fn add_y(&mut self, delta_y : f64) {
        let y = self.get_y();
        self.set_y(y + delta_y);
    }

    pub fn set_rotation(&mut self, rot : f64) {
        let c = rot.cos();
        let s = rot.sin();
        self.transform[0][0] = c;
        self.transform[0][1] = -s;
        self.transform[1][0] = s;
        self.transform[1][1] = c;
    }

    pub fn get_rotation(&self) -> f64 {
        self.transform[1][0].atan2(self.transform[0][0])
    }

    pub fn add_rotation(&mut self, delta_rotation : f64) {
        let rot = self.get_rotation();
        self.set_rotation(rot + delta_rotation);
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
