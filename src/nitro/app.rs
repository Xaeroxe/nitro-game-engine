use piston::window::WindowSettings;
use piston::event_loop::*;
use piston_window;
use piston_window::{
    PistonWindow,
    UpdateArgs,
    RenderArgs,
    Event,
    OpenGL,
    TextureSettings,
    Flip,
    Input,
};
use gfx_device_gl::Resources;
use nitro::input::Button;
use nitro::input::Axis;
use nitro::game_object::GameObject;
use nitro::texture::Texture;
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
    axes : Vec<Axis>,
}

impl App {
    pub fn new() -> App {
        let opengl = OpenGL::V3_2;
        App {
            game_objects: vec!(),
            updated_game_objects : vec!(),
            buttons_pressed : vec!(),
            axes : vec!(),
            window : WindowSettings::new(
                    "Nitro Engine",
                    [800, 600]
                )
                .opengl(opengl)
                .exit_on_esc(true)
                .build()
                .unwrap(),
        }
    }

    fn render(&mut self, e : &Event) {
        use graphics::*;

        const GREY: [f32; 4] = [0.8, 0.8, 0.8, 1.0];
        let game_objs = &self.game_objects;
        self.window.draw_2d(e, |c, gl| {
            // Clear the screen.
            clear(GREY, gl);
            for game_obj in game_objs {
                let (tex_width, tex_height) = game_obj.texture.get_raw().get_size();
                image(game_obj.texture.get_raw(),
                    c.transform
                    .append_transform(game_obj.transform.get_raw())
                    .trans(-(tex_width as f64)/2.0, -(tex_height as f64)/2.0),
                gl);
            }
        });
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
    }

    pub fn run(&mut self) {
        let mut events = self.window.events();
        while let Some(e) = events.next(&mut self.window) {
            match e {
                Event::Render(render_args) => {
                    self.render(&e);
                }
                Event::AfterRender(after_render_args) => {}
                Event::Update(update_args) => {
                    self.update(&update_args)
                }
                Event::Idle(idle_args) => {}
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

    pub fn add_axis(&mut self, axis : Axis) {
        self.axes.push(axis);
    }

    pub fn get_axis_value(&self, axis_name : &str) -> Option<f64> {
        if let Some(axis) = (&self.axes).into_iter().find(|&axis| axis.get_name() == axis_name) {
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
                return_value.set_raw_texture(texture);
            }
            Err(err) => {
                println!("Unable to load texture, {} Error: {:?}", texture_name, err);
            }
        }
        return_value
    }
}
