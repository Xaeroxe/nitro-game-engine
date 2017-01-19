mod axis;
pub use self::axis::Axis;

pub mod input;

pub use piston::input::Button;

pub mod keyboard {
    pub use piston::input::keyboard::Key;
}

pub mod mouse {
    pub use piston::input::mouse::MouseButton;
}

pub mod controller {
    pub use piston::input::controller::ControllerButton;
}
