use nphysics2d;
use std::f32;
use math::Vector;

pub type Transform = nphysics2d::math::Isometry<f32>;

pub trait TransformDirections {
    fn right(&self) -> Vector;
    fn up(&self) -> Vector;
}

impl TransformDirections for Transform {
    fn right(&self) -> Vector {
        let angle = self.rotation.angle();
        Vector::new(angle.cos(), angle.sin())

    }

    fn up(&self) -> Vector {
        let angle = self.rotation.angle();
        Vector::new(-angle.sin(), angle.cos())
    }
}
