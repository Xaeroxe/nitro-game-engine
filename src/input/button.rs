use input::keyboard::Key;
use input::mouse::MouseButton;

/// A digital input, can be made to behave like an analog axis using nitro::input::Axis.
#[derive(Eq, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Button {
    Keyboard(Key),
    Mouse(MouseButton),
}
