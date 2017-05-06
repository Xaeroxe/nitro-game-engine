use math::IntVector;
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
    pos: IntVector,
}

impl Mouse {
    pub fn pos(&self) -> IntVector {
        self.pos
    }

    pub fn set_show_cursor(&mut self, input: bool) {
        self.mouse_util.show_cursor(input);
    }
}

pub fn new(mouse_util: MouseUtil) -> Mouse {
    Mouse {
        mouse_util: mouse_util,
        pos: IntVector::new(0, 0),
    }
}

pub fn process_event(mouse: &mut Mouse, event: &Event) {
    match *event {
        Event::MouseMotion { x, y, .. } => {
            mouse.pos = IntVector::new(x as i32, y as i32);
        }
        _ => {}
    }
}
