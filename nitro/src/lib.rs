extern crate sdl2;
extern crate conrod;
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
    pub use input_private::input::Input;
    pub use input_private::keyboard;
    pub use input_private::mouse;
    pub use input_private::Button;
}


mod camera;
pub use camera::Camera;

pub type Vector = nphysics2d::math::Vector<f32>;

pub enum OptionAway<T> {
    Some(T),
    Away,
    None,
}

impl<T> From<Option<Option<T>>> for OptionAway<T> {
    fn from(result: Option<Option<T>>) -> OptionAway<T> {
        match result {
            Some(Some(result)) => OptionAway::Some(result),
            Some(None) => OptionAway::Away,
            None => OptionAway::None,
        }
    }
}

impl<'a, T> From<Option<&'a Option<T>>> for OptionAway<&'a T> {
    fn from(result: Option<&'a Option<T>>) -> OptionAway<&'a T> {
        match result {
            Some(inner) => {
                match inner.as_ref() {
                    Some(inner) => OptionAway::Some(inner),
                    None => OptionAway::Away,
                }
            }
            None => OptionAway::None,
        }
    }
}

impl<'a, T> From<Option<&'a mut Option<T>>> for OptionAway<&'a mut T> {
    fn from(result: Option<&'a mut Option<T>>) -> OptionAway<&'a mut T> {

        match result {
            Some(inner) => {
                match inner.as_mut() {
                    Some(inner) => OptionAway::Some(inner),
                    None => OptionAway::Away,
                }
            }
            None => OptionAway::None,
        }
    }
}

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
