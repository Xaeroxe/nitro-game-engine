use app::App;
use texture::Texture;
use transform::Transform;
use component::Component;
use component::Message;
use std::collections::BTreeMap;
use std::mem;
use physics::nphysics2d::object::{RigidBody, RigidBodyHandle};
use physics::nphysics2d::math::Matrix;
use physics::nalgebra::{Rotation2, Vector2, Vector1};

pub struct GameObject {
    pub transform: Transform,
    pub texture: Texture,
    body: Option<RigidBodyHandle<f64>>,
    // When searching for a component associated with a GameObject you will need to search both of
    // these collections. At any given time either one of them could contain the Component you are
    // searching for. As messages are distributed to components they are migrated from
    // components to messaged_components and once the distribution of a message is complete
    // the two collections are swapped.
    components: BTreeMap<i32, Box<Component>>, // These have not had a message sent (yet).
    messaged_components: BTreeMap<i32, Box<Component>>, // These have had a message sent.
}

impl GameObject {
    pub fn new(app: &mut App) -> GameObject {
        GameObject {
            transform: Transform::new(),
            components: BTreeMap::new(),
            messaged_components: BTreeMap::new(),
            texture: Texture::empty(app),
            body: None,
        }
    }

    pub fn update(&mut self, app: &mut App, delta_time: f64) {
        let message = &Message::Update { delta_time: delta_time };
        for key in self.components.keys().map(|x| *x).collect::<Vec<i32>>() {
            if let Some(mut component) = self.components.remove(&key) {
                component.receive_message(app, self, &message);
                self.messaged_components.insert(key, component);
            }
        }
        assert_eq!(self.components.len(), 0);
        mem::swap(&mut self.components, &mut self.messaged_components);
    }

    pub fn component_indices(&self) -> Vec<i32> {
        self.components.keys().map(|x| *x).collect::<Vec<i32>>()
    }

    pub fn get_component(&self, index: i32) -> Option<&Box<Component>> {
        self.components.get(&index)
    }

    pub fn add_component_at_index(&mut self, component: Box<Component>, index: i32) {
        self.components.insert(index, component);
    }

    pub fn add_component(&mut self, component: Box<Component>) {
        let new_key = self.components.keys().nth(0).unwrap_or(&1) - 1;
        self.components.insert(new_key, component);
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
