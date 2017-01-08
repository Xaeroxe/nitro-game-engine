use app::App;
use texture::Texture;
use transform::Transform;
use component::Component;
use component::Message;
use std::collections::BTreeMap;
use std::any::TypeId;
use physics::nphysics2d::object::{RigidBody, RigidBodyHandle};
use physics::nphysics2d::math::Matrix;
use physics::nalgebra::{Rotation2, Vector2, Vector1, Rotation};

pub struct GameObject {
    pub transform: Transform,
    pub texture: Texture,
    pub body: Option<RigidBodyHandle<f64>>,
    components: BTreeMap<i32, (Box<Component>, TypeId)>,
}

impl GameObject {
    pub fn new(app: &mut App) -> GameObject {
        GameObject {
            transform: Transform::new(),
            components: BTreeMap::new(),
            texture: Texture::empty(app),
            body: None,
        }
    }

    pub fn update(&mut self, app: &mut App, delta_time: f64) {
        let message = &Message::Update { delta_time: delta_time };
        for key in self.components.keys().map(|x| *x).collect::<Vec<i32>>() {
            if let Some((mut component, type_id)) = self.components.remove(&key) {
                component.receive_message(app, self, &message);
                self.components.insert(key, (component, type_id));
            }
        }
    }

    pub fn component_keys<'a>(&'a self) -> Box<Iterator<Item=i32>+'a> {
        Box::new(self.components.keys().map(|x| *x))
    }

    pub fn component_keys_with_type<'a, T>(&'a self) -> Box<Iterator<Item=i32>+'a> where T: Component + 'static {
        Box::new(self.components.iter().filter_map(|(k, &(_, type_id))| {
            if type_id == TypeId::of::<T>() {Some(*k)}
            else {None}
        }))
    }

    pub fn remove_component(&mut self, index: i32) -> Option<(Box<Component>, TypeId)> {
        self.components.remove(&index)
    }

    pub fn component(&self, index: i32) -> Option<&(Box<Component>, TypeId)> {
        self.components.get(&index)
    }

    pub fn component_mut(&mut self, index: i32) -> Option<&mut (Box<Component>, TypeId)> {
        self.components.get_mut(&index)
    }

    pub fn insert_component<T>(&mut self, component: T, index: i32)
    -> Option<(Box<Component + 'static>, TypeId)>
    where T: Component + 'static {
        self.components.insert(index, (Box::new(component), TypeId::of::<T>()))
    }

    pub fn add_component<T>(&mut self, component: T) -> i32 where T: Component + 'static {
        let new_key = self.components.keys().map(|x| *x).nth(0).unwrap_or(1) - 1;
        self.components.insert(new_key, (Box::new(component), TypeId::of::<T>()));
        new_key
    }

    pub fn set_rigid_body(&mut self, app: &mut App, rigid_body: RigidBody<f64>) {
        self.body = Some(app.world.add_rigid_body(rigid_body));
    }
}

pub fn copy_from_physics(game_object: &mut GameObject) {
    if let &Some(ref body_box) = &game_object.body {
        let body_borrow = body_box.borrow();
        *game_object.transform.mut_x() = body_borrow.position().translation.x;
        *game_object.transform.mut_y() = body_borrow.position().translation.y;
        *game_object.transform.mut_rotation() = body_borrow.position().rotation.rotation().x
    }
}

pub fn copy_to_physics(game_object: &mut GameObject) {
    if let Some(ref mut body_box) = game_object.body {
        body_box.borrow_mut().set_transformation(Matrix::<f64> {
            rotation: Rotation2::new(Vector1 { x: *game_object.transform.rotation() }),
            translation: Vector2 {
                x: *game_object.transform.x(),
                y: *game_object.transform.y(),
            },
        });
    }
}
