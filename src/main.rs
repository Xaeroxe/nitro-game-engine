extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate gfx_device_gl;

mod game_object;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston_window::{PistonWindow, OpenGL};
use game_object::GameObject;

pub struct App {
    window : PistonWindow,
    game_objects : Vec<GameObject>,
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
                let (tex_width, tex_height) = game_obj.texture.get_size();
                image(&game_obj.texture,
                    c.transform
                    .trans(game_obj.x, game_obj.y)
                    .rot_rad(game_obj.rot)
                    .trans(-(tex_width as f64)/2.0, -(tex_height as f64)/2.0),
                gl);
            }
        });
    }

    fn update(&mut self, args : &UpdateArgs) {
        for game_obj in &mut self.game_objects {
            game_obj.rot += 1.0 * args.dt;
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    // Create a new game and run it.
    let mut app = App {
        game_objects: vec!(),
        window : WindowSettings::new(
                "Nitro Engine",
                [800, 600]
            )
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap(),
    };
    let mut game_obj = GameObject::new(&mut app.window);
    game_obj.x = 400.0;
    game_obj.y = 300.0;
    game_obj.set_texture(&mut app.window, "nitro.png");
    app.game_objects.push(game_obj);
    let mut events = app.window.events();
    while let Some(e) = events.next(&mut app.window) {
        if let Some(r) = e.render_args() {
            app.render(&e);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
