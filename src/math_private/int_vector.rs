use nphysics2d;
use math::Vector;

pub type IntVector = nphysics2d::math::Vector<i32>;

pub trait IntVecConvert {
    fn to_vec(&self) -> Vector;
}

impl IntVecConvert for IntVector {
    fn to_vec(&self) -> Vector {
        Vector {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}
