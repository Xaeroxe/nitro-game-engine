mod axis;
pub use self::axis::Axis;

use piston_window::Button as PistonButton;
use num::FromPrimitive;

pub mod input;

mod input_enums;
pub use self::input_enums::keyboard;
pub use self::input_enums::controller;
pub use self::input_enums::mouse;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Button {
    Keyboard(self::keyboard::Key),
    Mouse(self::mouse::MouseButton),
    Controller(self::controller::ControllerButton),
}

impl From<PistonButton> for Button {
    fn from(pbutton: PistonButton) -> Button {
        match pbutton {
            PistonButton::Keyboard(key) => {
                Button::Keyboard(self::keyboard::Key::from_u32(key as u32).unwrap())
            }
            PistonButton::Mouse(button) => {
                Button::Mouse(self::mouse::MouseButton::from_u32(button as u32).unwrap())
            }
            PistonButton::Controller(button) => {
                Button::Controller(self::controller::ControllerButton {
                    id: button.id,
                    button: button.button,
                })
            }
        }
    }
}
