use piston::window::WindowSettings;
use piston_window;
use piston_window::{PistonWindow, UpdateArgs, Event, OpenGL, TextureSettings, Flip};
use gfx_device_gl::Resources;
use glutin;
use audio_private;
use audio_private::dj;
use audio::Dj;
use input_private;
use input::Input;
use game_object::GameObject;
use game_object;
use component::Message;
use texture::Texture;
use texture;
use PolarCoords;
use Vector;
use transform::Transform;
use transform;
use camera::Camera;
use nphysics2d::world::World;
use rodio;
use rodio::Decoder;
use rodio::source::Buffered;
use rodio::Source;
use std::collections::HashMap;
use std::borrow::BorrowMut;
use std::fs::File;
use std::io::BufReader;
use std::cell::RefCell;
use std::path::PathBuf;
use std::f32;

type BufferedAudioFile = Buffered<Decoder<BufReader<File>>>;

pub struct App {
    window: PistonWindow,
    game_objects: HashMap<u64, Box<GameObject>>,
    djs: HashMap<u64, Box<Dj>>,
    next_game_object_id: u64,
    next_dj_id: u64,
    pub input: Input,
    sound_cache: HashMap<String, Box<BufferedAudioFile>>,
    pub camera: Camera,
    pub world: World<f32>,
}

impl App {
    pub fn new(name: &str) -> App {
        let opengl = OpenGL::V3_2;
        let (width, height) = glutin::get_primary_monitor().get_dimensions();
        App {
            next_game_object_id: 0,
            next_dj_id: 0,
            game_objects: HashMap::new(),
            djs: HashMap::new(),
            input: Input::new(),
            sound_cache: HashMap::new(),
            window: WindowSettings::new(name, [width, height])
                .opengl(opengl)
                .exit_on_esc(true)
                .fullscreen(false)
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
                for game_obj in game_objs.values() {
                    let mut render_transform = game_obj.transform;
                    *render_transform.mut_x() -= *camera_transform.x();
                    *render_transform.mut_y() -= *camera_transform.y();
                    let mut polar = PolarCoords::from(render_transform.position().clone());
                    polar.rotation -= *camera_transform.rotation();
                    *render_transform.mut_position() = Vector::from(polar);
                    *render_transform.mut_x() -= (render_args.draw_width / 2) as f32;
                    *render_transform.mut_y() -= (render_args.draw_height / 2) as f32;
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
        //Process Djs for this frame.
        let keys = self.djs.keys().map(|x| *x).collect::<Vec<u64>>();
        for key in keys {
            if let Some(mut dj) = self.djs.remove(&key) {
                if dj.is_over() {
                    let (game_object_id, component_id) = dj::get_listener(&dj);
                    let message = Message::DjIdle { dj: RefCell::new(dj.borrow_mut()) };
                    if let Some(mut game_object) = self.game_objects.remove(&game_object_id) {
                        game_object.send_message_to_component(self, &message, component_id);
                        self.game_objects.insert(game_object.id(), game_object);
                    }
                }
                if !dj::was_dropped(&dj) {
                    self.djs.insert(dj.id(), dj);
                }
            }
        }
        //Copy game_object to the physics world, step, then copy from physics to game_object
        for mut game_object in self.game_objects.values_mut() {
            game_object::copy_to_physics(game_object);
        }
        self.world.step(args.dt as f32);
        for mut game_object in self.game_objects.values_mut() {
            game_object::copy_from_physics(game_object);
        }

        //Send update messages
        let keys = self.game_objects.keys().map(|x| *x).collect::<Vec<u64>>();
        for key in keys {
            if let Some(mut game_object) = self.game_objects.remove(&key) {
                game_object.update(self, args.dt as f32);
                *game_object.transform.mut_rotation() %= 2.0 * f32::consts::PI;
                if !game_object::was_dropped(game_object.borrow_mut()) {
                    self.game_objects.insert(game_object.id(), game_object);
                }
            }
        }
        input_private::input::shift_frame(&mut self.input);
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
                    input_private::input::process_event(&mut self.input, input_event);
                }
            }
        }
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn play_sound(&mut self, path: &str, volume: f32) {
        let mut sink = rodio::Sink::new(&rodio::get_default_endpoint().unwrap());
        sink.set_volume(volume);
        sink.append(fetch_sound(self, path));
        sink.detach();
    }

    pub fn new_dj<F>(&mut self, f: F) -> u64
        where F: FnOnce(&mut App, &mut Dj)
    {
        let sink = rodio::Sink::new(&rodio::get_default_endpoint().unwrap());
        let mut dj = Box::new(audio_private::dj::new_dj(sink, self));
        f(self, dj.borrow_mut());
        let id = dj.id();
        self.djs.insert(id, dj);
        id
    }

    pub fn dj_by_id(&self, id: u64) -> Option<&Dj> {
        if let Some(boxxed) = self.djs.get(&id) {
            Some(boxxed.as_ref())
        } else {
            None
        }
    }

    pub fn dj_by_id_mut(&mut self, id: u64) -> Option<&mut Dj> {
        if let Some(boxxed) = self.djs.get_mut(&id) {
            Some(boxxed.borrow_mut())
        } else {
            None
        }
    }

    // Load a sound into the cache if it's not already there.
    pub fn load_sound(&mut self, path: &str) {
        if !self.sound_cache.contains_key(path) {
            let mut sound_path = PathBuf::from("assets");
            sound_path.push("sounds");
            sound_path.push(path);
            let file = File::open(sound_path).unwrap();
            let new_source =
                Box::new(rodio::Decoder::new(BufReader::new(file)).unwrap().buffered());
            self.sound_cache.insert(path.to_owned(), new_source);
        }
    }

    pub fn unload_sound(&mut self, path: &str) {
        self.sound_cache.remove(path);
    }

    pub fn new_gameobject<F>(&mut self, f: F) -> u64
        where F: FnOnce(&mut App, &mut GameObject)
    {
        let mut game_object = Box::new(game_object::new(self));
        f(self, &mut game_object);
        let id = game_object.id();
        self.game_objects.insert(id, game_object);
        id
    }

    pub fn game_object_by_id(&self, id: u64) -> Option<&GameObject> {
        if let Some(boxxed) = self.game_objects.get(&id) {
            Some(boxxed.as_ref())
        } else {
            None
        }
    }

    pub fn game_object_by_id_mut(&mut self, id: u64) -> Option<&mut GameObject> {
        if let Some(boxxed) = self.game_objects.get_mut(&id) {
            Some(boxxed.borrow_mut())
        } else {
            None
        }
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

// This function will never return 0.  0 can now be used as a null value.
pub fn next_dj_id(app: &mut App) -> u64 {
    app.next_dj_id += 1;
    app.next_dj_id
}

// This function will never return 0.  0 can now be used as a null value.
pub fn next_game_object_id(app: &mut App) -> u64 {
    app.next_game_object_id += 1;
    app.next_game_object_id
}

// Fetches sound from cache if present, otherwise loads it from the filesystem.
pub fn fetch_sound(app: &mut App, path: &str) -> BufferedAudioFile {
    if !app.sound_cache.contains_key(path) {
        let mut sound_path = PathBuf::from("assets");
        sound_path.push("sounds");
        sound_path.push(path);
        let file = File::open(sound_path).unwrap();
        let new_source = rodio::Decoder::new(BufReader::new(file)).unwrap().buffered();
        app.sound_cache.insert(path.to_owned(), Box::new(new_source.clone()));
        return new_source;
    }
    (**app.sound_cache.get(path).unwrap()).clone()
}
