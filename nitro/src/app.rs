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
use input::Button;
use input::Axis;
use game_object::GameObject;
use texture::Texture;
use texture;
use transform::Transform;
use transform;
use camera::Camera;
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
    axes : HashMap<i32, Axis>,
    camera : Camera,
}

impl App {
    pub fn new(name : &str, exit_on_esc : bool) -> App {
        let opengl = OpenGL::V3_2;
        App {
            game_objects: vec!(),
            updated_game_objects : vec!(),
            buttons_pressed : vec!(),
            axes : HashMap::new(),
            window : WindowSettings::new(
                    name,
                    [800, 600]
                )
                .opengl(opengl)
                .exit_on_esc(exit_on_esc)
                .build()
                .unwrap(),
            camera : Camera{transform : Transform::new()},
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
            let camera_transform = transform::get_raw_inverse(&self.camera.transform);
            self.window.draw_2d(e, |c, gl| {
                // Clear the screen.
                clear(GREY, gl);
                for game_obj in game_objs {
                    let (tex_width, tex_height) = texture::get_raw(&game_obj.texture).get_size();
                    image(texture::get_raw(&game_obj.texture),
                        c.transform
                        .append_transform(camera_transform);
                        .append_transform(transform::get_raw(&game_obj.transform))
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
        *self.camera.transform.x() += 5.0 * args.dt;
    }

    pub fn run(&mut self) {
        let mut events = self.window.events();
        while let Some(e) = events.next(&mut self.window) {
            match e {
                Event::Render(render_args) => {
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
                            self.buttons_pressed.push(button);
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

    pub fn is_button_pressed(&self, button : Button) -> bool {
        (&self.buttons_pressed).into_iter().any(|&b| b == button)
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
