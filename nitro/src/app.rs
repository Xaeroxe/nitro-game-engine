use piston::window::WindowSettings;
use piston::event_loop::*;
use piston_window;
use piston_window::{
    PistonWindow,
    UpdateArgs,
    Event,
    OpenGL,
    TextureSettings,
    Flip,
    Input,
};
use gfx_device_gl::Resources;
use glutin;
use input::Button;
use input::Axis;
use game_object::GameObject;
use texture::Texture;
use texture;
use transform::Transform;
use transform;
use camera::Camera;
use liquidfun::box2d::common::math::*;
use liquidfun::box2d::dynamics::world::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::mem;

pub struct App {
    window : PistonWindow,
    // When searching for a GameObject you will need to search both of
    // these vectors. At any given time either one of them could contain the GameObject you are
    // searching for. As GameObjects are updated they are migrated from
    // game_objects to updated_game_objects and once the update is complete
    // the two vectors are swapped.
    game_objects : Vec<GameObject>,
    updated_game_objects : Vec<GameObject>,
    buttons_pressed : Vec<Button>,
    previous_buttons_pressed : Vec<Button>, // buttons_pressed from last frame.
    axes : HashMap<i32, Axis>,
    actions : HashMap<i32, Button>,
    camera : Camera,
    world : World,
}

impl App {
    pub fn new(name : &str, fullscreen : bool, exit_on_esc : bool) -> App {
        let opengl = OpenGL::V3_2;
        let (width, height) = glutin::get_primary_monitor().get_dimensions();
        App {
            game_objects: vec!(),
            updated_game_objects : vec!(),
            buttons_pressed : vec!(),
            previous_buttons_pressed : vec!(),
            axes : HashMap::new(),
            actions : HashMap::new(),
            window : WindowSettings::new(
                    name,
                    [width, height]
                )
                .opengl(opengl)
                .exit_on_esc(exit_on_esc)
                .fullscreen(fullscreen)
                .build()
                .unwrap(),
            camera : Camera{transform : Transform::new()},
            world : World::new(&Vec2::new(0.0, 10.0)),
        }
    }

    fn render(&mut self, e : &Event) {
        // This should never be false.
        // The only reason I didn't request the render_args in the signature is the Piston API
        // Requires the event object.
        if let Event::Render(render_args) = *e {
            use graphics::*;
            const GREY: [f32; 4] = [0.8, 0.8, 0.8, 1.0];
            let game_objs = &self.game_objects;
            let camera_transform = self.camera.transform;
            self.window.draw_2d(e, |c, gl| {
                // Clear the screen.
                clear(GREY, gl);
                for game_obj in game_objs {
                    let mut render_transform = game_obj.transform;
                    *render_transform.mut_x() -= *camera_transform.x() - (render_args.draw_width/2) as f64;
                    *render_transform.mut_y() -= *camera_transform.y() - (render_args.draw_height/2) as f64;
                    *render_transform.mut_rotation() -= *camera_transform.rotation();
                    let (tex_width, tex_height) = texture::get_raw(&game_obj.texture).get_size();
                    image(texture::get_raw(&game_obj.texture),
                        c.transform
                        .append_transform(transform::get_raw(&render_transform))
                        .trans(-(tex_width as f64)/2.0, -(tex_height as f64)/2.0),
                    gl);
                }
            });
        }
        else {
            panic!("Render should never be called with an event that isn't Event::Render.");
        }
    }

    fn update(&mut self, args : &UpdateArgs) {
        let mut pop_result = self.game_objects.pop();
        while let Some(mut game_object) = pop_result {
            game_object.update(self, args.dt);
            self.updated_game_objects.push(game_object);
            pop_result = self.game_objects.pop();
        }
        assert_eq!(self.game_objects.len(), 0);
        mem::swap(&mut self.game_objects, &mut self.updated_game_objects);
        self.previous_buttons_pressed = self.buttons_pressed.clone();
    }

    pub fn run(&mut self) {
        while let Some(e) = self.window.next() {
            match e {
                Event::Render(_) => {
                    self.render(&e);
                }
                Event::AfterRender(_) => {}
                Event::Update(update_args) => {
                    self.update(&update_args)
                }
                Event::Idle(_) => {}
                Event::Input(input_event) => {
                    match input_event {
                        Input::Press(button) => {
                            // The button may already be here as this event does repeat.
                            if !self.buttons_pressed.iter().any(|&item| item == button) {
                                self.buttons_pressed.push(button);
                            }
                        }
                        Input::Release(button) => {
                            while let Some(i) = self.buttons_pressed.iter().position(
                                |&item| item == button
                            )
                            {
                                self.buttons_pressed.swap_remove(i);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn add_axis(&mut self, axis : Axis, id : i32) {
        if self.axes.contains_key(&id) {
            panic!("Axis id values must be unique!");
        }
        self.axes.insert(id, axis);
    }

    pub fn get_axis_value(&self, id : i32) -> Option<f64> {
        if let Some(axis) = self.axes.get(&id) {
            return Some(axis.get_value(self));
        }
        None
    }

    pub fn add_action(&mut self, button : Button, id : i32) {
        if self.actions.contains_key(&id) {
            panic!("Action id values must be unique!");
        }
        self.actions.insert(id, button);
    }

    // Named for consistency with get_axis_value
    pub fn action_pressed(&self, id : i32) -> Option<bool> {
        if let Some(button) = self.actions.get(&id) {
            return Some(self.is_button_pressed(*button));
        }
        None
    }

    pub fn action_down(&self, id : i32) -> Option<bool> {
        if let Some(button) = self.actions.get(&id) {
            return Some(self.is_button_down(*button));
        }
        None
    }

    pub fn action_released(&self, id : i32) -> Option<bool> {
        if let Some(button) = self.actions.get(&id) {
            return Some(self.is_button_released(*button));
        }
        None
    }

    pub fn is_button_pressed(&self, button : Button) -> bool {
        (&self.buttons_pressed).into_iter().any(|&b| b == button)
        && !(&self.previous_buttons_pressed).into_iter().any(|&b| b == button)
    }

    pub fn is_button_down(&self, button : Button) -> bool {
        (&self.buttons_pressed).into_iter().any(|&b| b == button)
    }

    pub fn is_button_released(&self, button : Button) -> bool {
        !(&self.buttons_pressed).into_iter().any(|&b| b == button)
        && (&self.previous_buttons_pressed).into_iter().any(|&b| b == button)
    }

    pub fn add_gameobject(&mut self, game_object : GameObject) {
        self.game_objects.push(game_object);
    }

    pub fn empty_raw_texture(&mut self) -> piston_window::Texture<Resources> {
        piston_window::Texture::empty(&mut self.window.factory)
        .expect("Getting empty texture failed. 0_o")
    }

    pub fn fetch_texture(&mut self, texture_name : &str) -> Texture {
        let mut texture_path = PathBuf::from("assets");
        texture_path.push("textures");
        texture_path.push(texture_name);
        let result = piston_window::Texture::from_path(
            &mut self.window.factory,
            texture_path,
            Flip::None,
            &TextureSettings::new()
        );
        let mut return_value = Texture::empty(self);
        match result {
            Ok(texture) => {
                texture::set_raw_texture(&mut return_value, texture);
            }
            Err(err) => {
                println!("Unable to load texture, {} Error: {:?}", texture_name, err);
            }
        }
        return_value
    }
}
