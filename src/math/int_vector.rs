use nphysics2d;
use math::Vector;

pub type IntVector = nphysics2d::math::Vector<i32>;

pub trait IntVecConvert {
    fn to_vec(&self) -> Vector;
}

impl IntVecConvert for IntVector {
    fn to_vec(&self) -> Vector {
        Vector::new(self.x as f32, self.y as f32)
    }
}
