use math::IntVector;
use math::Vector;
use sdl2::event::Event;
use sdl2::mouse::MouseUtil;

// ***WARNING***:
// Many trait impls are highly dependent on the fact that this enum is identical to
// the keycodes enum from SDL2
enum_from_primitive! {
    /// Definitions for mouse buttons.  Intended to be used with nitro::input::Input.
    #[derive(Eq, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
    pub enum MouseButton {
        Unknown,
        Left,
        Right,
        Middle,
        X1,
        X2,
        Button6,
        Button7,
        Button8,
    }
}

/// Used to get input from the player's mouse, access from nitro::App.input.mouse
pub struct Mouse {
    mouse_util: MouseUtil,
    pos: Vector,
    relative_pos: Vector,
}

impl Mouse {
    pub fn pos(&self) -> Vector {
        self.pos
    }

    pub fn set_show_cursor(&mut self, input: bool) {
        self.mouse_util.show_cursor(input);
    }

    pub fn set_relative_mode(&mut self, input: bool) {
        self.mouse_util.set_relative_mouse_mode(input);
    }
}

pub fn new(mouse_util: MouseUtil) -> Mouse {
    Mouse {
        mouse_util: mouse_util,
        pos: Vector::new(0.0, 0.0),
        relative_pos: Vector::new(0.0, 0.0),
    }
}

pub fn process_event(mouse: &mut Mouse, event: &Event) {
    match *event {
        Event::MouseMotion{ x, y, .. } => {
            mouse.pos = Vector::new(x as f32, y as f32);
        }
        _ => {}
    }
}
