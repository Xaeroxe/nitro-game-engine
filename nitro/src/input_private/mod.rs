mod axis;
pub use self::axis::Axis;

use num::FromPrimitive;

pub mod input;

pub mod keyboard;
pub mod mouse;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Button {
    Keyboard(self::keyboard::Key),
    Mouse(self::mouse::MouseButton),
}

