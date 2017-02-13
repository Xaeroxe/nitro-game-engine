#[macro_use]
extern crate nitro;

mod spinny;
mod axes;
mod actions;

use nitro::App;
use nitro::input::{Axis, Button};
use nitro::input::keyboard::Key;
use nitro::Vector;
use spinny::Spinny;
use axes::AxisId;
use actions::ActionId;

fn main() {
    // Create a new game and run it.
    let mut app = App::new("Halera");

    app.input.add_axis(Axis::new(Button::Keyboard(Key::D), Button::Keyboard(Key::A)),
                       AxisId::Horizontal as i32);
    app.input.add_axis(Axis::new(Button::Keyboard(Key::S), Button::Keyboard(Key::W)),
                       AxisId::Vertical as i32);
    app.input.add_axis(Axis::new(Button::Keyboard(Key::E), Button::Keyboard(Key::Q)),
                       AxisId::Rotation as i32);
    app.input.add_action(Button::Keyboard(Key::F), ActionId::Blink as i32);

    app.input.save_bindings("halera.bindings");
    app.input.load_bindings("halera.bindings");
    app.new_gameobject(|app, game_obj| {
        game_obj.texture = app.fetch_texture("nitro.png");
        game_obj.add_component(app, Spinny {});
    });
    app.world.set_gravity(Vector { x: 0.0, y: 9.0 });
    app.run();
}
