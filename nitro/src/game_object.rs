use app;
use app::App;
use texture::Texture;
use transform::Transform;
use component::Component;

use component::Message;
use physics::nphysics2d::object::{RigidBody, RigidBodyHandle};
use physics::nphysics2d::math::Matrix;
use physics::nalgebra::{Rotation2, Vector2, Vector1, Rotation};

pub struct GameObject {
    pub transform: Transform,
    pub texture: Texture,
    pub body: Option<RigidBodyHandle<f32>>,
    // This value will never be 0.  0 can now be used as a null value.
    id: u64,
}

pub struct GameObjectId {
    id: u64,
}

impl GameObject {
    pub fn new(app: &mut App) -> GameObject {
        GameObject {
            id: 0,
            transform: Transform::new(),
            texture: Texture::empty(app),
            body: None,
        }
    }

    pub fn get_id(&self) -> GameObjectId {
        GameObjectId { id: self.id }
    }

    pub fn component_keys<'a>(&'a self) -> Box<Iterator<Item = i32> + 'a> {
        Box::new(self.components.keys().map(|x| *x))
    }

    pub fn component_keys_with_type<'a, T>(&'a self) -> Box<Iterator<Item = i32> + 'a>
        where T: Component + 'static
    {
        Box::new(self.components
            .iter()
            .filter_map(|(k, c)| if c.as_any().is::<T>() { Some(*k) } else { None }))
    }

    pub fn remove_component(&mut self, index: i32) -> Option<Box<ComponentAny>> {
        self.components.remove(&index)
    }

    pub fn component(&self, index: i32) -> Option<&Box<ComponentAny>> {
        self.components.get(&index)
    }

    pub fn component_mut(&mut self, index: i32) -> Option<&mut Box<ComponentAny>> {
        self.components.get_mut(&index)
    }

    pub fn component_with_type<T>(&self, index: i32) -> Option<&T>
        where T: Component + 'static
    {
        if let Some(component) = self.components.get(&index) {
            return component.as_any().downcast_ref::<T>();
        }
        None
    }

    pub fn component_mut_with_type<T>(&mut self, index: i32) -> Option<&mut T>
        where T: Component + 'static
    {
        if let Some(component) = self.components.get_mut(&index) {
            return component.as_any_mut().downcast_mut::<T>();
        }
        None
    }

    pub fn insert_component<T>(&mut self,
                               app: &mut App,
                               component: T,
                               index: i32)
                               -> Option<Box<ComponentAny>>
        where T: Component + 'static
    {
        let mut boxxed = Box::new(component);
        boxxed.receive_message(app, self, &Message::Start { key: index });
        self.components.insert(index, boxxed)
    }

    pub fn add_component<T>(&mut self, app: &mut App, component: T) -> i32
        where T: Component + 'static
    {
        // TODO: Figure out how to best get an app reference here.
        let new_key = self.components.keys().map(|x| *x).nth(0).unwrap_or(1) - 1;
        let mut boxxed = Box::new(component);
        boxxed.receive_message(app, self, &Message::Start { key: new_key });
        self.components.insert(new_key, boxxed);
        new_key
    }

    pub fn set_rigid_body(&mut self, app: &mut App, rigid_body: RigidBody<f32>) {
        self.body = Some(app.world.add_rigid_body(rigid_body));
    }
}

pub fn set_id(game_object: &mut GameObject, id: u64) {
    game_object.id = id;
}

pub fn copy_from_physics(game_object: &mut GameObject) {
    if let &Some(ref body_box) = &game_object.body {
        let body_borrow = body_box.borrow();
        *game_object.transform.mut_x() = body_borrow.position().translation.x;
        *game_object.transform.mut_y() = body_borrow.position().translation.y;
        *game_object.transform.mut_rotation() = body_borrow.position().rotation.rotation().x;
    }
}

pub fn copy_to_physics(game_object: &mut GameObject) {
    if let Some(ref mut body_box) = game_object.body {
        body_box.borrow_mut().set_transformation(Matrix::<f32> {
            rotation: Rotation2::new(Vector1 { x: *game_object.transform.rotation() }),
            translation: Vector2 {
                x: *game_object.transform.x(),
                y: *game_object.transform.y(),
            },
        });
    }
}
