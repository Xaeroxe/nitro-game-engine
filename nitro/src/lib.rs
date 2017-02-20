extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate gfx_device_gl;
extern crate glutin;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
#[macro_use]
extern crate unborrow;
#[macro_use]
extern crate enum_primitive;
extern crate num;
extern crate rodio;

pub extern crate ncollide;
pub extern crate nphysics2d;
pub extern crate nalgebra;

mod app;
pub use app::App;

mod audio_private;
pub mod audio {
    pub use audio_private::dj::Dj;
}

mod game_object;
pub use game_object::GameObject;

#[macro_use]
pub mod component;

mod texture;
pub use texture::Texture;

mod transform;
pub use transform::Transform;

mod polar_coords;
pub use polar_coords::PolarCoords;

mod input_private;
pub mod input {
    pub use input_private::Axis;
    pub use input_private::controller;
    pub use input_private::input::Input;
    pub use input_private::keyboard;
    pub use input_private::mouse;
    pub use input_private::Button;
}


mod camera;
pub use camera::Camera;

pub type Vector = nphysics2d::math::Vector<f32>;


use std::ops::SubAssign;
pub fn check_and_use<T, K>(resource: &mut T, cost: K) -> bool
    where T: SubAssign<K> + PartialOrd<K>
{
    if *resource >= cost {
        *resource -= cost;
        true
    } else {
        false
    }
}
