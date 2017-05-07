use sdl2;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;
use sdl2::render::Texture as SdlTexture;
use sdl2::mixer;
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
use graphics::Screen;
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
use nphysics2d::math::Translation;
use nalgebra::geometry::UnitComplex;
use specs::Planner;
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
    planner: Planner,    
}

pub struct AppData {
    delta_time: f32,
    exit: bool
}

impl AppData {
    /// Signal to the App that it's time to quit.  App will exit at the end of this frame.
    pub fn exit(&mut self) {
        self.exit = true;
    }

    /// Get the total time it took to compute the previous frame in seconds.
    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }
}

impl App {
    ///Constructs a new App using the given name for the window title.
    pub fn new(name: &str) -> App {
        let sdl_context = sdl2::init().expect("Failed to initialize SDL2.");
        let video_subsystem = sdl_context
            .video()
            .expect("Failed to initialize video subsystem.");
        let window = video_subsystem
            .window(name, 800, 600)
            .position_centered()
            .fullscreen_desktop()
            .opengl()
            .build()
            .expect("Failed to initialize SDL window.");
        let canvas = CanvasBuilder::new(window)
            .accelerated()
            .build()
            .expect("Failed to initialize canvas");
        let audio = sdl_context.audio().expect("Failed to initialize audio.");
        let mixer = mixer::init(mixer::INIT_OGG).expect("Failed to initialize mixer.");
        mixer::open_audio(mixer::DEFAULT_FREQUENCY, mixer::DEFAULT_FORMAT, 2, 1024)
            .expect("Failed to open audio.");
        mixer::allocate_channels(256);
        mixer::reserve_channels(128);
        let planner = {
            let world = World::new();
            world.add_resource(Screen::new(canvas));
            world.add_resource(audio::new(audio, mixer));
            world.add_resource(sdl_context.event_pump().expect("Failed to initialize event pump."));
            world.add_resource(Camera {
                transform: Transform::new(Vector::new(0.0, 0.0), 0.0),
                zoom: 1.0,
            });
            world.add_resource(input::new(sdl_context.mouse()));
            world.add_resource(AppData {delta_time: 0.0, exit: false});
            Planner::new(world)
        }
        App {
            planner
        }
    }


    /// Called every frame to paint the scene. Do not put game logic here, that goes in update.
    fn render(run_arg: RunArg) {
        
        let game_objs = &self.game_objects;
        let camera_transform = self.camera.transform;
        self.renderer.set_draw_color(Color::RGB(0, 0, 0));
        self.renderer.clear();
        let (screen_width, screen_height) = self.renderer.window().unwrap().size();
        let mut game_obj_sorted = game_objs.values().filter_map(|g| g.as_ref()).collect::<Vec<&Box<GameObject>>>();
        (&mut game_obj_sorted).sort_by_key(|g| g.draw_layer);
        for game_obj in game_obj_sorted {
            if let Some(ref sprite) = game_obj.sprite {
                let mut render_transform = game_obj.transform;
                render_transform.translation.vector.x -= camera_transform.translation.vector.x;
                render_transform.translation.vector.y -= camera_transform.translation.vector.y;
                let mut polar = PolarCoords::from(render_transform.translation.vector.clone());
                polar.rotation -= camera_transform.rotation.angle();
                render_transform.translation = Translation::from_vector(Vector::from(polar));
                render_transform.translation.vector.y *= -1.0;
                render_transform.translation.vector *= self.camera.zoom * self.screen_to_world_ratio;
                render_transform.translation.vector.x += (screen_width / 2) as f32;
                render_transform.translation.vector.y += (screen_height / 2) as f32;

                // Now we can determine if an object should be culled.
                if (render_transform.translation.vector.x as i32 + self.cull_padding as i32) > 0
                && (render_transform.translation.vector.x as i32 - self.cull_padding as i32) < screen_width as i32
                && (render_transform.translation.vector.y as i32 + self.cull_padding as i32) > 0
                && (render_transform.translation.vector.y as i32 - self.cull_padding as i32) < screen_height as i32 {
                    render_transform.rotation =
                    UnitComplex::from_angle(render_transform.rotation.angle() -
                                            camera_transform.rotation.angle());
                    match *sprite {
                        Sprite::Texture(ref texture) => {
                            let (tex_width, tex_height) = texture::size(texture);
                            let draw_width = (tex_width as f32 * self.camera.zoom) as u32;
                            let draw_height = (tex_height as f32 * self.camera.zoom) as u32;
                            let render_rect =
                                Rect::new((render_transform.translation.vector.x as i32) -
                                            (draw_width as i32 / 2),
                                            (render_transform.translation.vector.y as i32) -
                                            (draw_height as i32 / 2),
                                            draw_width,
                                            draw_height);
                            let result = self.renderer
                                .copy_ex(texture::get_raw(texture),
                                            None,
                                            Some(render_rect),
                                            ((camera_transform.rotation.angle() - game_obj.transform.rotation.angle()) * 180.0 / f32::consts::PI) as f64 ,
                                            None,
                                            texture.flip_horizontal,
                                            texture.flip_vertical);
                            if let Err(err) = result {
                                println!("Unable to draw texture, Error: {:?}", err);
                            }
                        }
                        Sprite::SpriteSheet(ref sprite_sheet) => {
                            let ref current_frame = sprite_sheet.animations
                                [sprite_sheet.current_animation as usize][sprite_sheet.current_frame as usize];
                            let draw_width = current_frame.frame_rect.width() as f32 * self.camera.zoom;
                            let draw_height = current_frame.frame_rect.height() as f32 * self.camera.zoom;
                            let result = self.renderer
                                .copy_ex(sprite_sheet::get_texture(sprite_sheet),
                                            Some(Rect::from(current_frame.frame_rect)),
                                            Some(Rect::new((render_transform.translation.vector.x - draw_width) as i32 / 2,
                                                        (render_transform.translation.vector.y - draw_height) as i32 / 2,
                                                        draw_width as u32,
                                                        draw_height as u32)),
                                            (game_obj.transform.rotation.angle() * 180.0 / f32::consts::PI) as f64,
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
        self.renderer.present();
    }

    /// Begin execution of the game.
    ///
    /// This function won't return until the game has been quit.
    /// If profiling is true then additional information on game processing durations will be
    /// printed to console.
    pub fn run(&mut self) {
        let mut last_frame_instant = Instant::now();
        while !self.planner.mut_world().read_resource_now<AppData>().exit {
            // Measure time elapsed in the previous frame.
            let now = Instant::now();
            let delta_time = Duration::from_std(last_frame_instant.elapsed())
                            .unwrap()
                            .num_microseconds()
                            .unwrap() as f32 / 1000000.0;
            last_frame_instant = now;


            // Event polling from SDL2 Window.
            self.planner.run_custom(|run_arg| {
                let event_pump;
                let input;
                let app_data;
                run_arg.fetch(|world| {
                    event_pump = world.write_resource<EventPump>();
                    input = world.write_resource<Input>();
                    app_data = world.write_resource<AppData>();
                });
                app_data.delta_time = delta_time;
                while let Some(e) = event_pump.poll_event() {
                    if let Event::Quit { .. } = e {
                        app_data.exit();
                    }
                    input_private::input::process_event(&mut input, &e);
                }
            });
            
            
            //Pass control to user here.
            
            //Render game
            self.planner.run_custom(render);
        }
    }

    /// Exit the game
    ///
    /// Signals that it's time to quit. Execution will stop at the end of the current frame.
    pub fn quit(&mut self) {
        self.exit = true;
    }

    pub fn world_mouse_pos(&self) -> Vector {
        let (screen_width, screen_height) = self.renderer.window().unwrap().size();
        let mut mouse_relative_pos = PolarCoords::from((self.input.mouse.pos() -
                                                        IntVector::new((screen_width / 2) as i32,
                                                                       (screen_height / 2) as
                                                                       i32))
                                                               .to_vec());
        mouse_relative_pos.rotation += self.camera.transform.rotation.angle();
        self.camera.transform.translation.vector + (Vector::from(mouse_relative_pos) / (self.camera.zoom * self.screen_to_world_ratio))
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
        self.texture_cache
            .insert(texture_name.to_string(), sdl_texture.clone());
        Ok(texture::new(sdl_texture))
    }
}
