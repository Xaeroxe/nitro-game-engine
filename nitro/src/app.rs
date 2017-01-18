use piston::window::WindowSettings;
use piston_window;
use piston_window::{PistonWindow, UpdateArgs, Event, OpenGL, TextureSettings, Flip, Input};
use gfx_device_gl::Resources;
use glutin;
use input::Button;
use input::Axis;
use game_object::GameObject;
use game_object;
use texture::Texture;
use texture;
use transform::Transform;
use transform;
use camera::Camera;
use physics::nphysics2d::world::World;
use serde_hjson;
use rodio;
use rodio::Decoder;
use rodio::source::Buffered;
use rodio::Source;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::collections::LinkedList;
use std::path::PathBuf;
use std::f32;
use std::mem;

pub struct App {
    window: PistonWindow,
    // When searching for a GameObject you will need to search both of
    // these vectors. At any given time either one of them could contain the GameObject you are
    // searching for. As GameObjects are updated they are migrated from
    // game_objects to updated_game_objects and once the update is complete
    // the two vectors are swapped.
    game_objects: LinkedList<GameObject>,
    updated_game_objects: LinkedList<GameObject>,
    buttons_pressed: Vec<Button>,
    previous_buttons_pressed: Vec<Button>, // buttons_pressed from last frame.
    axes: HashMap<i32, Axis>,
    actions: HashMap<i32, Button>,
    sound_cache: HashMap<String, Box<Buffered<Decoder<BufReader<File>>>>>,
    pub camera: Camera,
    pub world: World<f32>,
}

impl App {
    pub fn new(name: &str) -> App {
        let opengl = OpenGL::V3_2;
        let (width, height) = glutin::get_primary_monitor().get_dimensions();
        App {
            game_objects: LinkedList::new(),
            updated_game_objects: LinkedList::new(),
            buttons_pressed: vec![],
            previous_buttons_pressed: vec![],
            axes: HashMap::new(),
            actions: HashMap::new(),
            sound_cache: HashMap::new(),
            window: WindowSettings::new(name, [width, height])
                .opengl(opengl)
                .exit_on_esc(true)
                .fullscreen(true)
                .build()
                .unwrap(),
            camera: Camera { transform: Transform::new() },
            world: World::new(),
        }
    }

    fn render(&mut self, e: &Event) {
        // This should never be false.
        // The only reason I didn't request the render_args in the signature is the Piston API
        // requires the event object.
        if let Event::Render(render_args) = *e {
            use graphics::*;
            const GREY: [f32; 4] = [0.9, 0.9, 0.9, 1.0];
            let game_objs = &self.game_objects;
            let camera_transform = self.camera.transform;
            self.window.draw_2d(e, |c, gl| {
                // Clear the screen.
                clear(GREY, gl);
                for game_obj in game_objs {
                    let mut render_transform = game_obj.transform;
                    *render_transform.mut_x() -= *camera_transform.x() -
                                                 (render_args.draw_width / 2) as f32;
                    *render_transform.mut_y() -= *camera_transform.y() -
                                                 (render_args.draw_height / 2) as f32;
                    *render_transform.mut_rotation() -= *camera_transform.rotation();
                    let (tex_width, tex_height) = texture::get_raw(&game_obj.texture).get_size();
                    image(texture::get_raw(&game_obj.texture),
                          c.transform
                              .append_transform(transform::get_raw(&render_transform))
                              .trans(-(tex_width as f64) / 2.0, -(tex_height as f64) / 2.0),
                          gl);
                }
            });
        } else {
            panic!("Render should never be called with an event that isn't Event::Render.");
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        for mut game_object in &mut self.game_objects {
            game_object::copy_to_physics(&mut game_object);
        }
        self.world.step(args.dt as f32);
        for mut game_object in &mut self.game_objects {
            game_object::copy_from_physics(&mut game_object);
        }
        let mut pop_result = self.game_objects.pop_front();
        while let Some(mut game_object) = pop_result {
            game_object.update(self, args.dt);
            *game_object.transform.mut_rotation() %= 2.0 * f32::consts::PI;
            self.updated_game_objects.push_back(game_object);
            pop_result = self.game_objects.pop_front();
        }
        assert_eq!(self.game_objects.len(), 0);
        mem::swap(&mut self.game_objects, &mut self.updated_game_objects);
        self.previous_buttons_pressed = self.buttons_pressed.clone();
    }

    pub fn run(&mut self) {
        while let Some(e) = self.window.next() {
            match e {
                Event::Render(..) => {
                    self.render(&e);
                }
                Event::AfterRender(..) => {}
                Event::Update(update_args) => self.update(&update_args),
                Event::Idle(..) => {}
                Event::Input(input_event) => {
                    match input_event {
                        Input::Press(button) => {
                            // The button may already be here as this event does repeat.
                            if !self.buttons_pressed.iter().any(|&item| item == button) {
                                self.buttons_pressed.push(button);
                            }
                        }
                        Input::Release(button) => {
                            while let Some(i) = self.buttons_pressed
                                .iter()
                                .position(|&item| item == button) {
                                self.buttons_pressed.swap_remove(i);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn play_sound(&mut self, path: &str) {
        let source;

        if self.sound_cache.contains_key(path) {
            let cached = self.sound_cache.get(path).unwrap();
            source = cached.clone();
        } else {
            let mut sound_path = PathBuf::from("assets");
            sound_path.push("sounds");
            sound_path.push(path);
            let file = File::open(sound_path).unwrap();
            let new_source =
                Box::new(rodio::Decoder::new(BufReader::new(file)).unwrap().buffered());
            source = new_source.clone();
            self.sound_cache.insert(path.to_owned(), new_source);
        }
        let endpoint = rodio::get_default_endpoint().unwrap();
        let sink = rodio::Sink::new(&endpoint);
        sink.append(source.buffered());
    }

    pub fn load_sound(&mut self, path: &str) {
        let mut sound_path = PathBuf::from("assets");
        sound_path.push("sounds");
        sound_path.push(path);
        let file = File::open(sound_path).unwrap();
        let new_source = Box::new(rodio::Decoder::new(BufReader::new(file)).unwrap().buffered());
        self.sound_cache.insert(path.to_owned(), new_source);
    }

    pub fn unload_sound(&mut self, path: &str) {
        self.sound_cache.remove(path);
    }

    pub fn save_bindings(&mut self, path: &str) -> Result<(), String> {
        for axis in &self.axes {
            match serde_hjson::ser::to_string(&axis) {
                Ok(result) => {
                    println!("{}", result);
                }
                Err(err) => {
                    return Err(format!("{:?}", err));
                }
            }
        }
        Ok(())
    }

    pub fn add_axis(&mut self, axis: Axis, id: i32) {
        if self.axes.contains_key(&id) {
            panic!("Axis id values must be unique!");
        }
        self.axes.insert(id, axis);
    }

    pub fn get_axis_value(&self, id: i32) -> Option<f64> {
        if let Some(axis) = self.axes.get(&id) {
            return Some(axis.get_value(self));
        }
        None
    }

    pub fn add_action(&mut self, button: Button, id: i32) {
        if self.actions.contains_key(&id) {
            panic!("Action id values must be unique!");
        }
        self.actions.insert(id, button);
    }

    // Named for consistency with get_axis_value
    pub fn action_pressed(&self, id: i32) -> Option<bool> {
        if let Some(button) = self.actions.get(&id) {
            return Some(self.is_button_pressed(*button));
        }
        None
    }

    pub fn action_down(&self, id: i32) -> Option<bool> {
        if let Some(button) = self.actions.get(&id) {
            return Some(self.is_button_down(*button));
        }
        None
    }

    pub fn action_released(&self, id: i32) -> Option<bool> {
        if let Some(button) = self.actions.get(&id) {
            return Some(self.is_button_released(*button));
        }
        None
    }

    pub fn is_button_pressed(&self, button: Button) -> bool {
        (&self.buttons_pressed).into_iter().any(|&b| b == button) &&
        !(&self.previous_buttons_pressed).into_iter().any(|&b| b == button)
    }

    pub fn is_button_down(&self, button: Button) -> bool {
        (&self.buttons_pressed).into_iter().any(|&b| b == button)
    }

    pub fn is_button_released(&self, button: Button) -> bool {
        !(&self.buttons_pressed).into_iter().any(|&b| b == button) &&
        (&self.previous_buttons_pressed).into_iter().any(|&b| b == button)
    }

    pub fn add_gameobject(&mut self, game_object: GameObject) {
        self.game_objects.push_front(game_object);
    }

    pub fn empty_raw_texture(&mut self) -> piston_window::Texture<Resources> {
        piston_window::Texture::empty(&mut self.window.factory)
            .expect("Getting empty texture failed. 0_o")
    }

    pub fn fetch_texture(&mut self, texture_name: &str) -> Texture {
        let mut texture_path = PathBuf::from("assets");
        texture_path.push("textures");
        texture_path.push(texture_name);
        let result = piston_window::Texture::from_path(&mut self.window.factory,
                                                       texture_path,
                                                       Flip::None,
                                                       &TextureSettings::new());
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
