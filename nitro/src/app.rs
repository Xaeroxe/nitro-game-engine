use sdl2;
use sdl2::AudioSubsystem;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;
use sdl2::mixer;
use sdl2::mixer::Sdl2MixerContext;
use sdl2::mixer::Channel;
use sdl2::mixer::Chunk;
use OptionAway;
use input_private;
use input::Input;
use game_object::GameObject;
use component::Message;
use game_object;
use texture::Texture;
use texture;
use PolarCoords;
use Vector;
use transform::Transform;
use camera::Camera;
use nphysics2d::world::World;
use std::mem::replace;
use std::collections::HashMap;
use std::borrow::Borrow;
use std::path::PathBuf;
use std::f32;
use std::time::Instant;
use chrono::Duration;

/// This structure represents a game instance.
///
/// This structure is responsible for managing asset loading,
/// processing input, rendering the game and playing audio.
///
/// This structure makes 128 sound channels available to the user of this library.
/// When member methods request a channel id that id must be between 0 and 127.
pub struct App {
    exit: bool,
    renderer: Renderer<'static>,
    _audio: AudioSubsystem,
    _mixer: Sdl2MixerContext,
    event_pump: EventPump,
    game_objects: HashMap<u64, Option<Box<GameObject>>>,
    sound_cache: HashMap<String, Chunk>,
    next_game_object_id: u64,
    pub input: Input,
    pub camera: Camera,
    pub world: World<f32>,
}

impl App {
    ///Constructs a new App using the given name for the window title.
    pub fn new(name: &str) -> App {
        let sdl_context = sdl2::init().expect("Failed to initialize SDL2.");
        let video_subsystem = sdl_context.video().expect("Failed to initialize video subsystem");
        let window = video_subsystem.window(name, 800, 600)
            .position_centered()
            .fullscreen_desktop()
            .opengl()
            .build()
            .unwrap();
        let renderer = window.renderer().build().expect("Failed to initialize renderer");
        let audio = sdl_context.audio().expect("Failed to initialize audio");
        let mixer = mixer::init(mixer::INIT_OGG).expect("Failed to initialize mixer");
        mixer::open_audio(mixer::DEFAULT_FREQUENCY, mixer::DEFAULT_FORMAT, 2, 1024)
            .expect("Failed to open audio");
        mixer::allocate_channels(256);
        mixer::reserve_channels(128);
        App {
            exit: false,
            next_game_object_id: 0,
            game_objects: HashMap::new(),
            input: Input::new(),
            renderer: renderer,
            sound_cache: HashMap::new(),
            _audio: audio,
            _mixer: mixer,
            event_pump: sdl_context.event_pump().expect("Failed to initalize event pump."),
            camera: Camera { transform: Transform::new() },
            world: World::new(),
        }
    }


    /// Called every frame to paint the scene. Do not put game logic here, that goes in update.
    fn render(&mut self) {
        use std::f32;
        let game_objs = &self.game_objects;
        let camera_transform = self.camera.transform;
        // Clear the screen with grey.
        self.renderer.set_draw_color(Color::RGB(240, 240, 240));
        self.renderer.clear();
        // Reset the draw color to white so subsequent draw calls are correct.
        self.renderer.set_draw_color(Color::RGB(255, 255, 255));
        let (screen_width, screen_height) = self.renderer.window().unwrap().size();
        for game_obj in game_objs.values() {
            if let Some(ref game_obj) = *game_obj {
                let mut render_transform = game_obj.transform;
                *render_transform.mut_x() -= *camera_transform.x();
                *render_transform.mut_y() -= *camera_transform.y();
                let mut polar = PolarCoords::from(render_transform.position().clone());
                polar.rotation -= *camera_transform.rotation();
                *render_transform.mut_position() = Vector::from(polar);
                *render_transform.mut_x() += (screen_width / 2) as f32;
                *render_transform.mut_y() += (screen_height / 2) as f32;
                *render_transform.mut_rotation() -= *camera_transform.rotation();
                let (tex_width, tex_height) = texture::size(&game_obj.texture);
                let render_rect =
                    Rect::new(((*render_transform.x()) as i32) - (tex_width as i32 / 2),
                              ((*render_transform.y()) as i32) - (tex_height as i32 / 2),
                              tex_width,
                              tex_height);
                if let &Some(ref texture) = texture::get_raw(&game_obj.texture) {
                    let result = self.renderer.copy_ex(&texture,
                                                       None,
                                                       Some(render_rect),
                                                       (*game_obj.transform.rotation() * 180.0 /
                                                        f32::consts::PI) as
                                                       f64,
                                                       None,
                                                       false,
                                                       false);
                    if let Err(err) = result {
                        println!("Unable to draw texture, Error: {:?}", err);
                    }
                }
            }
        }
        self.renderer.present();
    }


    /// Called every frame to simulate the game. Do not put rendering here, that goes in render.
    fn update(&mut self, delta_time: f32) {
        //Copy game_object to the physics world, step, then copy from physics to game_object
        for mut game_object in self.game_objects.values_mut() {
            if let Some(game_object) = game_object.as_mut() {
                game_object::copy_to_physics(game_object);
            }
        }
        self.world.step(delta_time);
        for mut game_object in self.game_objects.values_mut() {
            if let Some(game_object) = game_object.as_mut() {
                game_object::copy_from_physics(game_object);
            }
        }

        //Send update messages
        let keys = self.game_objects.keys().map(|x| *x).collect::<Vec<u64>>();
        for key in keys {
            let mut game_obj_option = None;
            if let Some(game_obj_ref) = self.game_objects.get_mut(&key) {
                game_obj_option = replace(game_obj_ref, None);
            }
            if let Some(ref mut game_obj) = game_obj_option {
                game_obj.update(self, delta_time);
                *game_obj.transform.mut_rotation() %= 2.0 * f32::consts::PI;
            }
            if let Some(game_obj_ref) = self.game_objects.get_mut(&key) {
                replace(game_obj_ref, game_obj_option);
            }
        }

        // Drop any game objects that need to be dropped.
        let mut dropped_keys = Vec::new();
        for (key, game_obj) in self.game_objects.iter() {
            if let Some(ref game_obj) = *game_obj {
                if game_object::was_dropped(game_obj.borrow()) {
                    dropped_keys.push(*key);
                }
            } else {
                dropped_keys.push(*key);
            }
        }
        for key in dropped_keys {
            let removed = self.game_objects.remove(&key);
            if let Some(Some(mut game_obj)) = removed {
                game_obj.receive_message(self, &Message::OnDestroy);
            }
        }

        input_private::input::shift_frame(&mut self.input);
    }

    /// Begin execution of the game.
    ///
    /// This function won't return until the game has been quit.
    pub fn run(&mut self) {
        mixer::allocate_channels(128);
        let mut last_frame_instant = Instant::now();
        while !self.exit {
            while let Some(e) = self.event_pump.poll_event() {
                if let Event::Quit { .. } = e {
                    self.exit = true;
                }
                input_private::input::process_event(&mut self.input, &e);
            }
            self.update(Duration::from_std(last_frame_instant.elapsed())
                .unwrap()
                .num_microseconds()
                .unwrap() as f32 / 1000000.0);
            last_frame_instant = Instant::now();
            self.render();
        }
    }

    /// Exit the game
    ///
    /// Signals that it's time to quit. Execution will stop at the end of the current frame.
    pub fn quit(&mut self) {
        self.exit = true;
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    /// Plays a sound at a volume between 1.0 and 0.0
    ///
    /// path is a filename relative to assets/sounds (assets\sounds on Windows)
    pub fn play_sound(&mut self, path: &str, volume: f32) -> Result<(), String> {
        if let Some(chunk) = self.sound_cache.get(path) {
            let channel = Channel::all().play(chunk, 0)?;
            channel.set_volume((volume * 128.0) as i32);
            return Ok(());
        }
        let mut file_path = PathBuf::from("assets");
        file_path.push("sounds");
        file_path.push(path);
        let chunk = Chunk::from_file(file_path)?;
        let channel = Channel::all().play(&chunk, 0)?;
        channel.set_volume((volume * 128.0) as i32);
        self.sound_cache.insert(path.to_owned(), chunk);
        Ok(())
    }

    /// Play a sound on a user sound channel.
    pub fn play_sound_on_channel(&mut self, channel_id: i32, path: &str) -> Result<(), String> {
        if channel_id >= 0 && channel_id < 128 {
            if let Some(chunk) = self.sound_cache.get(path) {
                mixer::channel(channel_id).play(chunk, 0)?;
                return Ok(());
            }
            let mut file_path = PathBuf::from("assets");
            file_path.push("sounds");
            file_path.push(path);
            let chunk = Chunk::from_file(file_path)?;
            Channel::all().play(&chunk, 0)?;
            self.sound_cache.insert(path.to_owned(), chunk);
            return Ok(());
        }
        Err("Channel out of range.".to_string())
    }

    /// Set the volume for a user sound channel. Volume is between 0.0 and 1.0
    pub fn set_channel_volume(&mut self, channel_id: i32, volume: f32) -> Result<(), String> {
       if channel_id >= 0 && channel_id < 128 {
            mixer::channel(channel_id).set_volume((volume * 128.0) as i32);
            return Ok(());
       }
       Err("Channel out of range.".to_string())
    }

    /// Get the volume for a user sound channel. Volume is between 0.0 and 1.0
    pub fn get_channel_volume(&self, channel_id: i32) -> Result<f32, String> {
        if channel_id >= 0 && channel_id < 128 {
            return Ok((mixer::channel(channel_id).get_volume() as f32)/128.0);
        }
        Err("Channel out of range.".to_string())
    }

    /// Pause audio output on a user sound channel.
    pub fn pause_channel(&mut self, channel_id: i32) -> Result<(), String> {
        if channel_id >= 0 && channel_id < 128 {
            mixer::channel(channel_id).pause();
            return Ok(());
        }
        Err("Channel out of range.".to_string())
    }

    /// Resume paused audio output on a user sound channel.
    pub fn resume_channel(&mut self, channel_id: i32) -> Result<(), String> {
        if channel_id >= 0 && channel_id < 128 {
            mixer::channel(channel_id).resume();
            return Ok(());
        }
        Err("Channel out of range.".to_string())
    }

    /// Returns true if a channel is not playing anything. This will still return false if the
    /// channel has a sound to play but is paused.
    pub fn channel_idle(&mut self, channel_id: i32) -> Option<bool> {
        if channel_id >= 0 && channel_id < 128 {
            return Some(mixer::channel(channel_id).is_playing());
        }
        None
    }

    /// Creates a new GameObject
    ///
    /// GameObjects are typically physical objects in your game world, such as characters or
    /// decorative objects.
    ///
    /// Parameter f will be called on the newly created GameObject, allowing you to initialize it.
    /// Returns the id of the new GameObject, which can be used to retrieve this GameObject later.
    pub fn new_gameobject<F>(&mut self, f: F) -> u64
        where F: FnOnce(&mut App, &mut GameObject)
    {
        let mut game_object = Box::new(game_object::new(self));
        f(self, &mut game_object);
        let id = game_object.id();
        self.game_objects.insert(id, Some(game_object));
        id
    }

    /// Retrieves a reference to an existing GameObject by the GameObject's id.
    ///
    /// Returns Away if and only if the GameObject exists but is currently loaned out
    /// in another borrow.
    ///
    /// Hint: That borrow is most likely in the game_object parameter of the receive_message
    /// function.
    pub fn game_object_by_id(&self, id: u64) -> OptionAway<&GameObject> {
        OptionAway::from(self.game_objects.get(&id))
    }

    /// Retrieves a mutable reference to an existing GameObject by the GameObject's id.
    ///
    /// Returns Away if and only if the GameObject exists but is currently loaned out
    /// in another borrow.
    ///
    /// Hint: That borrow is most likely in the game_object parameter of the receive_message
    /// function.
    pub fn game_object_by_id_mut(&mut self, id: u64) -> OptionAway<&mut GameObject> {
        OptionAway::from(self.game_objects.get_mut(&id))
    }

    /// Loads a texture and returns it for use.
    ///
    /// texture_name is the file name of the texture relative to assets/textures
    /// (assets\textures on Windows)
    pub fn fetch_texture(&mut self, texture_name: &str) -> Result<Texture, String> {
        let mut texture_path = PathBuf::from("assets");
        texture_path.push("textures");
        texture_path.push(texture_name);
        let sdl_texture = self.renderer.load_texture(texture_path.as_path())?;
        let mut nitro_texture = Texture::new();
        texture::set_raw(&mut nitro_texture, sdl_texture);
        Ok(nitro_texture)
    }
}

// This function will never return 0.  0 can now be used as a null value.
pub fn next_game_object_id(app: &mut App) -> u64 {
    app.next_game_object_id += 1;
    app.next_game_object_id
}
