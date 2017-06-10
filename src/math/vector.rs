use nphysics2d;
use math::IntVector;

pub type Vector = nphysics2d::math::Vector<f32>;

pub trait VecConvert {
    fn to_int_vec(&self) -> IntVector;
}

impl VecConvert for Vector {
    fn to_int_vec(&self) -> IntVector {
        IntVector::new(self.x as i32, self.y as i32)
    }
}
