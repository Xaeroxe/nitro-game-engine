extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate gfx_device_gl;

mod game_object;
mod transform;
mod texture;
mod component;
mod spinny;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston_window::{PistonWindow, OpenGL};
use game_object::GameObject;
use spinny::Spinny;

pub struct App {
    window : PistonWindow,
    game_objects : Vec<GameObject>,
    buttons_pressed : Vec<Button>,
}

impl App {
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
        for game_obj in &mut self.game_objects {
            game_obj.update(args.dt);
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    // Create a new game and run it.
    let mut app = App {
        game_objects: vec!(),
        buttons_pressed : vec!(),
        window : WindowSettings::new(
                "Nitro Engine",
                [800, 600]
            )
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap(),
    };
    let mut game_obj = GameObject::new(&mut app);
    game_obj.transform.set_x(400.0);
    game_obj.transform.set_y(300.0);
    game_obj.texture.set_texture(&mut app, "nitro.png");
    game_obj.add_component(Box::new(Spinny{}));
    app.game_objects.push(game_obj);
    let mut events = app.window.events();
    while let Some(e) = events.next(&mut app.window) {
        match e {
            Event::Render(render_args) => {
                app.render(&e);
            }
            Event::AfterRender(after_render_args) => {}
            Event::Update(update_args) => {
                app.update(&update_args)
            }
            Event::Idle(idle_args) => {}
            Event::Input(input_event) => {
                match input_event {
                    Input::Press(button) => {
                        app.buttons_pressed.push(button);
                    }
                    Input::Release(button) => {
                        while let Some(i) = app.buttons_pressed.iter().position(
                            |&item| item == button
                        )
                        {
                            app.buttons_pressed.swap_remove(i);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
