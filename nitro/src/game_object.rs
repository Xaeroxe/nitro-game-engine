use app::App;
use texture::Texture;
use transform::Transform;
use component::Component;
use component::Message;
use std::collections::BTreeMap;
use std::any::Any;
use std::any::TypeId;
use physics::nphysics2d::object::{RigidBody, RigidBodyHandle};
use physics::nphysics2d::math::Matrix;
use physics::nalgebra::{Rotation2, Vector2, Vector1};

pub struct GameObject {
    pub transform: Transform,
    pub texture: Texture,
    body: Option<RigidBodyHandle<f64>>,
    components: BTreeMap<i32, Box<Component>>,
    component_types: BTreeMap<i32, TypeId>, // Identical to components just stores their type instead.
}

impl GameObject {
    pub fn new(app: &mut App) -> GameObject {
        GameObject {
            transform: Transform::new(),
            components: BTreeMap::new(),
            component_types: BTreeMap::new(),
            texture: Texture::empty(app),
            body: None,
        }
    }

    pub fn update(&mut self, app: &mut App, delta_time: f64) {
        let message = &Message::Update { delta_time: delta_time };
        for key in self.components.keys().map(|x| *x).collect::<Vec<i32>>() {
            if let Some(mut component) = self.components.remove(&key) {
                component.receive_message(app, self, &message);
                self.components.insert(key, component);
            }
        }
    }

    pub fn component_indices(&self) -> Vec<i32> {
        self.components.keys().map(|x| *x).collect::<Vec<i32>>()
    }

    pub fn get_components_of_type<T>(&self) -> Vec<i32> where T: Component + 'static {
        self.component_types.iter().filter_map(|(k, v)| {
            if *v == TypeId::of::<T>() {Some(*k)}
            else {None}
        }).collect::<Vec<i32>>()
    }

    pub fn get_component(&mut self, index: i32) -> Option<&mut Box<Component>> {
        self.components.get_mut(&index)
    }

    pub fn add_component_at_index(&mut self, component: Box<Component>, index: i32) {
        self.components.insert(index, component);
    }

    pub fn add_component<T>(&mut self, component: T) -> i32 where T: Component + 'static {
        let new_key = self.components.keys().nth(0).unwrap_or(&1) - 1;
        self.component_types.insert(new_key, TypeId::of::<T>());
        self.components.insert(new_key, Box::new(component));
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
        let matrix = body_borrow.position().rotation.submatrix();
        let y = matrix.m12;
        let x = matrix.m11;
        *game_object.transform.mut_rotation() = y.atan2(x);
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
