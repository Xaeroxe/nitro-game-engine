use app;
use app::App;
use OptionLoaned;
use graphics::Sprite;
use math::Transform;
use component::Component;
use component::ComponentAny;
use component::Message;
use std::collections::BTreeMap;
use std::mem::replace;
use nphysics2d::object::{RigidBody, RigidBodyHandle};
use nphysics2d::math::Matrix;
use nalgebra::{Rotation2, Vector2, Vector1, Rotation};

pub struct GameObject {
    pub transform: Transform,
    pub sprite: Option<Sprite>,
    pub body: Option<RigidBodyHandle<f32>>,
    // This value will never be 0.  0 can now be used as a null value.
    id: u64,
    components: BTreeMap<i32, Option<Box<ComponentAny>>>,
    drop: bool,
}

impl GameObject {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn drop(&mut self) {
        self.drop = true;
    }

    pub fn send_message_to_component(&mut self,
                                     app: &mut App,
                                     message: &Message,
                                     component_id: i32) {
        let mut component_option = None;
        if let Some(component_ref) = self.components.get_mut(&component_id) {
            component_option = replace(component_ref, None);
        }
        if let Some(ref mut component) = component_option {
            component.receive_message(app, self, &message);
        }
        if let Some(component_ref) = self.components.get_mut(&component_id) {
            replace(component_ref, component_option);
        }
    }

    pub fn receive_message(&mut self, app: &mut App, message: &Message) {
        for key in self.components
                .keys()
                .map(|x| *x)
                .collect::<Vec<i32>>() {
            self.send_message_to_component(app, message, key);
        }
    }

    pub fn update(&mut self, app: &mut App, delta_time: f32) {
        let message = &Message::Update { delta_time: delta_time };
        self.receive_message(app, message)
    }

    pub fn component_keys<'a>(&'a self) -> Box<Iterator<Item = i32> + 'a> {
        Box::new(self.components.keys().map(|x| *x))
    }

    pub fn component_keys_with_type<'a, T>(&'a self) -> Box<Iterator<Item = i32> + 'a>
        where T: Component + 'static
    {
        Box::new(self.components.iter().filter_map(|(k, c)| if let &Some(ref c) = c {
                                                       if c.as_any().is::<T>() {
                                                           Some(*k)
                                                       } else {
                                                           None
                                                       }
                                                   } else {
                                                       None
                                                   }))
    }

    pub fn remove_component(&mut self,
                            app: &mut App,
                            index: i32)
                            -> OptionLoaned<Box<ComponentAny>> {
        let mut component_result = self.components.remove(&index);
        if let Some(Some(ref mut component)) = component_result {
            component.receive_message(app, self, &Message::OnDetach);
        }
        OptionLoaned::from(component_result)
    }

    pub fn component(&self, index: i32) -> OptionLoaned<&Box<ComponentAny>> {
        OptionLoaned::from(self.components.get(&index))
    }

    pub fn component_mut(&mut self, index: i32) -> OptionLoaned<&mut Box<ComponentAny>> {
        OptionLoaned::from(self.components.get_mut(&index))
    }

    pub fn component_with_type<T>(&self, index: i32) -> Option<&T>
        where T: Component + 'static
    {
        if let Some(&Some(ref component)) = self.components.get(&index) {
            return component.as_any().downcast_ref::<T>();
        }
        None
    }

    pub fn component_mut_with_type<T>(&mut self, index: i32) -> Option<&mut T>
        where T: Component + 'static
    {
        if let Some(&mut Some(ref mut component)) = self.components.get_mut(&index) {
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
        let replaced = self.components.insert(index, Some(boxxed));
        if let Some(Some(component)) = replaced {
            Some(component)
        } else {
            None
        }
    }

    pub fn add_component<T>(&mut self, app: &mut App, component: T) -> i32
        where T: Component + 'static
    {
        let new_key = self.components
            .keys()
            .map(|x| *x)
            .nth(0)
            .unwrap_or(1) - 1;
        self.insert_component(app, component, new_key);
        new_key
    }

    pub fn set_rigid_body(&mut self, app: &mut App, rigid_body: RigidBody<f32>) {
        self.body = Some(app.world.add_rigid_body(rigid_body));
    }
}

pub fn new(app: &mut App) -> GameObject {
    GameObject {
        id: app::next_game_object_id(app),
        drop: false,
        transform: Transform::new(),
        components: BTreeMap::new(),
        sprite: None,
        body: None,
    }
}

pub fn was_dropped(game_object: &GameObject) -> bool {
    game_object.drop
}

pub fn copy_from_physics(game_object: &mut GameObject) {
    if let &Some(ref body_box) = &game_object.body {
        let body_borrow = body_box.borrow();
        *game_object.transform.mut_x() = body_borrow.position().translation.x;
        *game_object.transform.mut_y() = body_borrow.position().translation.y;
        *game_object.transform.mut_rotation() = body_borrow.position()
            .rotation
            .rotation()
            .x;
    }
}

pub fn copy_to_physics(game_object: &mut GameObject) {
    if let Some(ref mut body_box) = game_object.body {
        body_box.borrow_mut()
            .set_transformation(Matrix::<f32> {
                                    rotation: Rotation2::new(Vector1 {
                                                                 x: *game_object.transform
                                                                         .rotation(),
                                                             }),
                                    translation: Vector2 {
                                        x: *game_object.transform.x(),
                                        y: *game_object.transform.y(),
                                    },
                                });
    }
}
