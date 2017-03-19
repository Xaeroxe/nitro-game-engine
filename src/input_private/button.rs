use input::keyboard::Key;
use input::mouse::MouseButton;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Button {
    Keyboard(Key),
    Mouse(MouseButton),
}
