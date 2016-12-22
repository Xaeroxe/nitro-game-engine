use graphics::math::Matrix2d;
use std::f64;
use vec2::Vec2;

#[derive(Copy, Clone)]
pub struct Transform {
    position: Vec2,
    rotation: f64,
}

impl Transform {
    pub fn new() -> Transform {
        // zero format for Matrix2d. Reverse engineered from Piston.
        // x = 0, y = 0, rotation = 0
        Transform {
            position: Vec2 { x: 0.0, y: 0.0 },
            rotation: 0.0,
        }
    }

    pub fn from_x_y_rotation(x: f64, y: f64, rotation: f64) -> Transform {
        let mut value = Transform::new();
        *value.mut_x() = x;
        *value.mut_y() = y;
        *value.mut_rotation() = rotation;
        value
    }

    pub fn position(&self) -> &Vec2 {
        &self.position
    }

    pub fn mut_position(&mut self) -> &mut Vec2 {
        &mut self.position
    }

    pub fn x(&self) -> &f64 {
        &self.position.x
    }

    pub fn y(&self) -> &f64 {
        &self.position.y
    }

    pub fn rotation(&self) -> &f64 {
        &self.rotation
    }

    pub fn mut_x(&mut self) -> &mut f64 {
        &mut self.position.x
    }

    pub fn mut_y(&mut self) -> &mut f64 {
        &mut self.position.y
    }

    pub fn mut_rotation(&mut self) -> &mut f64 {
        self.rotation %= 2.0 * f64::consts::PI;
        &mut self.rotation
    }

    pub fn forward_vec2(&self) -> Vec2 {
        Vec2 {
            x: -self.rotation().sin(),
            y: self.rotation().cos(),
        }
    }

    pub fn right_vec2(&self) -> Vec2 {
        Vec2 {
            x: self.rotation().cos(),
            y: self.rotation().sin(),
        }
    }
}

// Raw format reverse engineered from Piston internals.
pub fn get_raw(nitro_transform: &Transform) -> Matrix2d {
    let c = nitro_transform.rotation.cos();
    let s = nitro_transform.rotation.sin();
    [[c, -s, nitro_transform.position.x], [s, c, nitro_transform.position.y]]
}
