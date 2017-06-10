use app::App;
use OptionLoaned;
use graphics::Sprite;
use math::{Transform, Vector};
use component::{Component, ComponentAny, Message};
use std::collections::BTreeMap;
use std::mem::replace;
use nphysics2d::object::{RigidBody, RigidBodyHandle};

/// GameObjects are the primary descriptor for things that are visible in the game world.
/// Players, items, enemies, and whatever else exists in your game world are typically GameObjects.
///
/// To create a new GameObject use nitro::App::new_gameobject().
pub struct GameObject {
    pub transform: Transform,
    pub sprite: Option<Sprite>,
    pub body: Option<RigidBodyHandle<f32>>,
    pub draw_layer: i32,
    // This value will never be 0.  0 can now be used as a null value.
    id: u64,
    components: BTreeMap<i32, Option<Box<ComponentAny>>>,
    // If this value is false messages will not be sent.  Optimization for game_objects with empty component lists.
    pub(crate) has_components: bool,
    pub(crate) drop: bool,
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
        if self.has_components {
            for key in self.components
                    .keys()
                    .map(|x| *x)
                    .collect::<Vec<i32>>() {
                self.send_message_to_component(app, message, key);
            }
        }
        
    }

    pub fn update(&mut self, app: &mut App, delta_time: f32) {
        if self.has_components {
            let message = &Message::Update { delta_time: delta_time };
            self.receive_message(app, message)
        }
    }

    pub fn component_keys<'a>(&'a self) -> Box<Iterator<Item = i32> + 'a> {
        Box::new(self.components.keys().map(|x| *x))
    }

    pub fn component_keys_with_type<'a, T>(&'a self) -> Box<Iterator<Item = i32> + 'a>
        where T: Component + 'static
    {
        Box::new(self.components
                     .iter()
                     .filter_map(|(k, c)| if let &Some(ref c) = c {
                                     if c.as_any().is::<T>() { Some(*k) } else { None }
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
        if self.components.len() == 0 {
            self.has_components = false;
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
        self.has_components = true;
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
        let new_key = self.components.keys().map(|x| *x).nth(0).unwrap_or(1) - 1;
        self.insert_component(app, component, new_key);
        new_key
    }

    pub fn set_rigid_body(&mut self, app: &mut App, rigid_body: RigidBody<f32>) {
        self.body = Some(app.world.add_rigid_body(rigid_body));
    }

    pub(crate) fn new(app: &mut App) -> GameObject {
        GameObject {
            id: app.next_game_object_id(),
            drop: false,
            transform: Transform::new(Vector::new(0.0, 0.0), 0.0),
            components: BTreeMap::new(),
            sprite: None,
            body: None,
            draw_layer: 0,
            has_components: false,
        }
    }

    pub(crate) fn copy_from_physics(&mut self) {
        if let &Some(ref body_box) = &self.body {
            let body_borrow = body_box.borrow();
            self.transform = body_borrow.position().clone();
        }
    }

    pub(crate) fn copy_to_physics(&mut self) {
        if let Some(ref mut body_box) = self.body {
            body_box
                .borrow_mut()
                .set_transformation(self.transform.clone());
        }
    }  
}


