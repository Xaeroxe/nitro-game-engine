#[macro_use]
extern crate nitro;

mod spinny;
mod axes;
mod actions;

use nitro::App;
use nitro::GameObject;
use nitro::input::{Axis, Button};
use nitro::input::keyboard::Key;
use nitro::Vector;
use spinny::Spinny;
use axes::AxisId;
use actions::ActionId;

fn main() {
    // Create a new game and run it.
    let mut app = App::new("Halera");
    app.input.add_axis(Axis::new("Horizontal", Button::Keyboard(Key::D), Button::Keyboard(Key::A)),
                       AxisId::Horizontal as i32);
    app.input.add_axis(Axis::new("Vertical", Button::Keyboard(Key::S), Button::Keyboard(Key::W)),
                       AxisId::Vertical as i32);
    app.input.add_axis(Axis::new("Rotation", Button::Keyboard(Key::E), Button::Keyboard(Key::Q)),
                       AxisId::Rotation as i32);
    app.input.add_action(Button::Keyboard(Key::F), ActionId::Blink as i32);
    app.input.save_bindings("halera.bindings");
    let mut game_obj = GameObject::new(&mut app);
    game_obj.texture = app.fetch_texture("nitro.png");
    game_obj.add_component(&mut app, Spinny {});
    app.add_gameobject(game_obj);
    app.world.set_gravity(Vector { x: 0.0, y: 9.0 });
    app.run();
}
