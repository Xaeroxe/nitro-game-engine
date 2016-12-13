use super::App;
use texture::Texture;
use transform::Transform;
use std::path::PathBuf;
use update_component::UpdateComponent;
use std::mem;

pub struct GameObject {
    pub transform : Transform,
    pub texture : Texture,
    update_components : Vec<Box<UpdateComponent>>, //These have not had update run this frame (yet).
    updated_components: Vec<Box<UpdateComponent>>, //These have had update run this frame.
}

impl GameObject {
    pub fn new(app : &mut App) -> GameObject {
        GameObject{
            transform : Transform::new(),
            update_components : Vec::new(),
            updated_components : Vec::new(),
            texture : Texture::empty(app),
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
}
