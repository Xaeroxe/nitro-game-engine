use graphics::math::Matrix2d;
use std::f32;
use Vector;

#[derive(Copy, Clone)]
pub struct Transform {
    position: Vector,
    rotation: f32,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            position: Vector { x: 0.0, y: 0.0 },
            rotation: 0.0,
        }
    }

    pub fn from_x_y_rotation(x: f32, y: f32, rotation: f32) -> Transform {
        let mut value = Transform::new();
        *value.mut_x() = x;
        *value.mut_y() = y;
        *value.mut_rotation() = rotation;
        value
    }

    pub fn position(&self) -> &Vector {
        &self.position
    }

    pub fn mut_position(&mut self) -> &mut Vector {
        &mut self.position
    }

    pub fn x(&self) -> &f32 {
        &self.position.x
    }

    pub fn y(&self) -> &f32 {
        &self.position.y
    }

    pub fn rotation(&self) -> &f32 {
        &self.rotation
    }

    pub fn mut_x(&mut self) -> &mut f32 {
        &mut self.position.x
    }

    pub fn mut_y(&mut self) -> &mut f32 {
        &mut self.position.y
    }

    pub fn mut_rotation(&mut self) -> &mut f32 {
        &mut self.rotation
    }

    pub fn forward_vector(&self) -> Vector {
        Vector {
            x: -self.rotation().sin(),
            y: self.rotation().cos(),
        }
    }

    pub fn right_vector(&self) -> Vector {
        Vector {
            x: self.rotation().cos(),
            y: self.rotation().sin(),
        }
    }
}

// Raw format reverse engineered from Piston internals.
pub fn get_raw(nitro_transform: &Transform) -> Matrix2d {
    let c = nitro_transform.rotation.cos() as f64;
    let s = nitro_transform.rotation.sin() as f64;
    [[c, -s, nitro_transform.position.x as f64], [s, c, nitro_transform.position.y as f64]]
}
