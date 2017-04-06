use sdl2;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;
use sdl2::render::Texture as SdlTexture;
use sdl2::mixer;
use OptionLoaned;
use input_private;
use input::Input;
use input_private::input;
use game_object::GameObject;
use component::Message;
use game_object;
use graphics::Texture;
use graphics_private::texture;
use graphics::Sprite;
use graphics_private::sprite_sheet;
use math::PolarCoords;
use math::Vector;
use math::IntVector;
use math_private::int_vector::IntVecConvert;
use audio::Audio;
use audio_private::audio;
use audio_private::playlist;
use math::Transform;
use graphics::Canvas;
use camera::Camera;
use nphysics2d::world::World;
use nphysics2d::math::Translation;
use nalgebra::geometry::UnitComplex;
use std::sync::Arc;
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
pub struct App {
    exit: bool,
    renderer: Renderer<'static>,
    event_pump: EventPump,
    game_objects: HashMap<u64, Option<Box<GameObject>>>,
    texture_cache: HashMap<String, Arc<SdlTexture>>,
    next_game_object_id: u64,
    pub input: Input,
    pub camera: Camera,
    pub world: World<f32>,
    pub audio: Audio,
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
        let input = input::new(sdl_context.mouse());
        App {
            exit: false,
            next_game_object_id: 0,
            game_objects: HashMap::new(),
            input: input,
            renderer: renderer,
            texture_cache: HashMap::new(),
            audio: audio::new(audio, mixer),
            event_pump: sdl_context.event_pump().expect("Failed to initalize event pump."),
            camera: Camera { transform: Transform::new(Vector::new(0.0, 0.0), 0.0) },
            world: World::new(),
        }
    }


    /// Called every frame to paint the scene. Do not put game logic here, that goes in update.
    fn render(&mut self) {
        use std::f32;
        let game_objs = &self.game_objects;
        let camera_transform = self.camera.transform;
        // Clear the screen with grey.
        self.renderer.set_draw_color(Color::RGB(0, 0, 0));
        self.renderer.clear();
        let (screen_width, screen_height) = self.renderer
            .window()
            .unwrap()
            .size();
        for game_obj in game_objs.values() {
            if let Some(ref game_obj) = *game_obj {
                if let Some(ref sprite) = game_obj.sprite {
                    let mut render_transform = game_obj.transform;
                    render_transform.translation.vector.x -= camera_transform.translation.vector.x;
                    render_transform.translation.vector.y -= camera_transform.translation.vector.y;
                    let mut polar = PolarCoords::from(render_transform.translation.vector.clone());
                    polar.rotation -= camera_transform.rotation.angle();
                    render_transform.translation = Translation::from_vector(Vector::from(polar));
                    render_transform.translation.vector.x += (screen_width / 2) as f32;
                    render_transform.translation.vector.y += (screen_height / 2) as f32;
                    render_transform.rotation = UnitComplex::from_angle(render_transform.rotation.angle() - camera_transform.rotation.angle());
                    match *sprite {
                        Sprite::Texture(ref texture) => {
                            let (tex_width, tex_height) = texture::size(texture);
                            let render_rect = Rect::new((render_transform.translation.vector.x as i32) -
                                                        (tex_width as i32 / 2),
                                                        (render_transform.translation.vector.y as i32) -
                                                        (tex_height as i32 / 2),
                                                        tex_width,
                                                        tex_height);
                            let result = self.renderer.copy_ex(texture::get_raw(texture),
                                                               None,
                                                               Some(render_rect),
                                                               (game_obj.transform.rotation.angle() *
                                                                180.0 /
                                                                f32::consts::PI) as
                                                               f64,
                                                               None,
                                                               texture.flip_horizontal,
                                                               texture.flip_vertical);
                            if let Err(err) = result {
                                println!("Unable to draw texture, Error: {:?}", err);
                            }
                        }
                        Sprite::SpriteSheet(ref sprite_sheet) => {
                            let ref current_frame = sprite_sheet.animations[sprite_sheet.current_animation as
                            usize]
                                [sprite_sheet.current_frame as usize];
                            let result = self.renderer
                                .copy_ex(sprite_sheet::get_texture(sprite_sheet),
                                         Some(Rect::from(current_frame.frame_rect)),
                                         Some(Rect::new((render_transform.translation.vector.x as i32) -
                                                        (current_frame.frame_rect.width() as i32 /
                                                         2),
                                                        (render_transform.translation.vector.y as i32) -
                                                        (current_frame.frame_rect.height() as i32 /
                                                         2),
                                                        current_frame.frame_rect.width(),
                                                        current_frame.frame_rect.height())),
                                         (game_obj.transform.rotation.angle() * 180.0 /
                                          f32::consts::PI) as
                                         f64,
                                         None,
                                         current_frame.flip_horizontal,
                                         current_frame.flip_vertical);
                            if let Err(err) = result {
                                println!("Unable to draw texture, Error: {:?}", err);
                            }
                        }
                    }
                }
            }
        }
        {
            let mut canvas = Canvas::new(&mut self.renderer);
            for game_obj in game_objs.values() {
                if let Some(ref game_obj) = *game_obj {
                    for key in game_obj.component_keys() {
                        if let OptionLoaned::Some(component) = game_obj.component(key) {
                            component.render_gui(&mut canvas, game_obj);
                        }
                    }
                }
            }
        }
        self.renderer.present();
    }


    /// Called every frame to simulate the game. Do not put rendering here, that goes in render.
    fn update(&mut self, delta_time: f32) {
        // Advance audio if necessary.
        playlist::advance_if_needed(&mut self.audio.playlist);
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
        let keys = self.game_objects
            .keys()
            .map(|x| *x)
            .collect::<Vec<u64>>();
        for key in keys {
            let mut game_obj_option = None;
            if let Some(game_obj_ref) = self.game_objects.get_mut(&key) {
                game_obj_option = replace(game_obj_ref, None);
            }
            if let Some(ref mut game_obj) = game_obj_option {
                game_obj.update(self, delta_time);
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

    pub fn world_mouse_pos(&self) -> Vector {
         let (screen_width, screen_height) = self.renderer
            .window()
            .unwrap()
            .size();
        self.camera.transform.translation.vector + (self.input.mouse.pos() - IntVector::new(screen_width as i32, screen_height as i32)).to_vec()
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
    pub fn game_object_by_id(&self, id: u64) -> OptionLoaned<&GameObject> {
        OptionLoaned::from(self.game_objects.get(&id))
    }

    /// Retrieves a mutable reference to an existing GameObject by the GameObject's id.
    ///
    /// Returns Away if and only if the GameObject exists but is currently loaned out
    /// in another borrow.
    ///
    /// Hint: That borrow is most likely in the game_object parameter of the receive_message
    /// function.
    pub fn game_object_by_id_mut(&mut self, id: u64) -> OptionLoaned<&mut GameObject> {
        OptionLoaned::from(self.game_objects.get_mut(&id))
    }

    /// Loads a texture and returns it for use.
    ///
    /// texture_name is the file name of the texture relative to assets/textures
    /// (assets\textures on Windows)
    pub fn fetch_texture(&mut self, texture_name: &str) -> Result<Texture, String> {
        if let Some(sdl_texture) = self.texture_cache.get(texture_name) {
            return Ok(texture::new(sdl_texture.clone()));
        }
        let mut texture_path = PathBuf::from("assets");
        texture_path.push("textures");
        texture_path.push(texture_name);
        let sdl_texture = self.renderer.load_texture(texture_path.as_path())?;
        let sdl_texture = Arc::new(sdl_texture);
        self.texture_cache.insert(texture_name.to_string(), sdl_texture.clone());
        Ok(texture::new(sdl_texture))
    }
}

// This function will never return 0.  0 can now be used as a null value.
pub fn next_game_object_id(app: &mut App) -> u64 {
    app.next_game_object_id += 1;
    app.next_game_object_id
}
