extern crate sdl2;
extern crate serde;
extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
#[macro_use]
extern crate unborrow;
#[macro_use]
extern crate enum_primitive;
extern crate num;

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

/// Special variant of Option.
///
/// TL; DR if a function returns Away that indicates the object is already loaned out
/// and is likely already available to you in the arguments of the function you're working in.
///
/// In order to satisfy Rust's rule that there cannot be multiple mutable aliases
/// to a struct and still maintain a convenient API items are removed from their
/// parent containers before they are passed to the user of Nitro, they are then
/// reinserted into their parent containers after the user is done with it.
/// In order to signal that an item still exists but is merely not in the queried
/// container at this instant the Away option may be returned by the API.
pub enum OptionAway<T> {
    Some(T),
    Away,
    None,
}

use std::borrow::{Borrow, BorrowMut};

impl<T> From<Option<Option<T>>> for OptionAway<T> {
    fn from(result: Option<Option<T>>) -> OptionAway<T> {
        match result {
            Some(Some(result)) => OptionAway::Some(result),
            Some(None) => OptionAway::Away,
            None => OptionAway::None,
        }
    }
}

impl<'a, T, M> From<Option<&'a Option<M>>> for OptionAway<&'a T>
    where M: Borrow<T>
{
    fn from(result: Option<&'a Option<M>>) -> OptionAway<&'a T> {
        match result {
            Some(inner) => {
                match inner.as_ref() {
                    Some(inner) => OptionAway::Some(inner.borrow()),
                    None => OptionAway::Away,
                }
            }
            None => OptionAway::None,
        }
    }
}

impl<'a, T, M> From<Option<&'a mut Option<M>>> for OptionAway<&'a mut T>
    where M: BorrowMut<T>
{
    fn from(result: Option<&'a mut Option<M>>) -> OptionAway<&'a mut T> {
        match result {
            Some(inner) => {
                match inner.as_mut() {
                    Some(inner) => OptionAway::Some(inner.borrow_mut()),
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
