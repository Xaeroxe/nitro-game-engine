use app::App;
use texture::Texture;
use transform::Transform;
use component::Component;
use component::Message;
use std::collections::LinkedList;
use std::mem;
use nphysics2d::object::RigidBodyHandle;
use nphysics2d::math::Matrix;
use nalgebra::{Rotation2, Vector2, Vector1};

pub struct GameObject {
    pub transform: Transform,
    pub texture: Texture,
    body: Option<RigidBodyHandle<f64>>,
    // When searching for a component associated with a GameObject you will need to search both of
    // these vectors. At any given time either one of them could contain the Component you are
    // searching for. As messages are distributed to components they are migrated from
    // components to messaged_components and once the distribution of a message is complete
    // the two vectors are swapped.
    components: LinkedList<Box<Component>>, // These have not had a message sent (yet).
    messaged_components: LinkedList<Box<Component>>, // These have had a message sent.
}

impl GameObject {
    pub fn new(app: &mut App) -> GameObject {
        GameObject {
            transform: Transform::new(),
            components: LinkedList::new(),
            messaged_components: LinkedList::new(),
            texture: Texture::empty(app),
            body: None,
        }
    }

    pub fn update(&mut self, app: &mut App, delta_time: f64) {
        let mut pop_result = self.components.pop_front();
        while let Some(mut component) = pop_result {
            component.receive_message(app, self, &Message::Update { delta_time: delta_time });
            self.messaged_components.push_back(component);
            pop_result = self.components.pop_front();
        }
        assert_eq!(self.components.len(), 0);
        mem::swap(&mut self.components, &mut self.messaged_components);
    }

    pub fn add_component(&mut self, component: Box<Component>) {
        self.components.push_front(component);
    }
}

pub fn copy_to_physics(game_object: &mut GameObject) {
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

pub fn copy_from_physics(game_object: &mut GameObject) {
    if let Some(ref mut body_box) = game_object.body {
        body_box.borrow_mut().set_transformation(
            Matrix::<f64> {
                rotation: Rotation2::new(Vector1 {x: *game_object.transform.rotation()}),
                translation: Vector2{x: *game_object.transform.x(), y: *game_object.transform.y()}
            }
        );
    }
}
