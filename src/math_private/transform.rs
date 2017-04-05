use std::f32;
use math::Vector;

#[derive(Copy, Clone, Debug)]
pub struct Transform {
    position: Vector,
    rotation: f32,
}

impl Transform {
    pub fn new(position: Vector, rotation: f32) -> Transform {
        Transform {
            position: position,
            rotation: rotation,
        }
    }

    /// Constructs a new transform from the given x position, y position, and rotation in radians.
    pub fn from_x_y_rot(x: f32, y: f32, rotation: f32) -> Transform {
        Transform::new(Vector::new(x, y), rotation)
    }

    pub fn position(&self) -> &Vector {
        &self.position
    }

    pub fn mut_position(&mut self) -> &mut Vector {
        &mut self.position
    }

    /// X component of the position.
    pub fn x(&self) -> &f32 {
        &self.position.x
    }

    /// Y component of the position.
    pub fn y(&self) -> &f32 {
        &self.position.y
    }

    /// Rotation of this transform in radians
    pub fn rotation(&self) -> &f32 {
        &self.rotation
    }

    /// X component of the position.
    pub fn mut_x(&mut self) -> &mut f32 {
        &mut self.position.x
    }

    /// Y component of the position.
    pub fn mut_y(&mut self) -> &mut f32 {
        &mut self.position.y
    }

    /// Rotation of this transform in radians
    pub fn mut_rotation(&mut self) -> &mut f32 {
        &mut self.rotation
    }

    /// This vector points forward relative to the Transform's rotation
    ///
    /// Useful for making an object go forward.  Multiply by -1 to get backwards.
    /// This direction could also be called up.  It's a matter of perspective.
    pub fn forward(&self) -> Vector {
        Vector::new(-self.rotation().sin(), self.rotation().cos())
    }

    /// This vector points to the right relative to the Transform's rotation
    ///
    /// Useful for making an object go to its right.  Multiply by -1 to get left.
    pub fn right(&self) -> Vector {
        Vector::new(self.rotation().cos(), self.rotation().sin())
    }
}
