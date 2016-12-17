use graphics::math::Matrix2d;
use std::f64;

#[derive(Copy, Clone)]
pub struct Transform {
    x : f64,
    y : f64,
    rotation : f64,
}

impl Transform {
    pub fn new() -> Transform {
        // zero format for Matrix2d. Reverse engineered from Piston.
        // x = 0, y = 0, rotation = 0
        Transform{x:0.0,y:0.0,rotation:0.0}
    }

    pub fn from_x_y_rotation(x : f64, y : f64, rotation : f64) -> Transform {
        let mut value = Transform::new();
        *value.x() = x;
        *value.y() = y;
        *value.rotation() = rotation;
        value
    }

    pub fn x(&mut self) -> &mut f64 {
        &mut self.x
    }

    pub fn y(&mut self) -> &mut f64 {
        &mut self.y
    }

    pub fn rotation(&mut self) -> &mut f64 {
        &mut self.rotation
    }
}

//Raw format reverse engineered from Piston internals.
pub fn get_raw(nitro_transform : &Transform) -> Matrix2d {
    let c = nitro_transform.rotation.cos();
    let s = nitro_transform.rotation.sin();
    [
        [c,-s,nitro_transform.x],
        [s,c,nitro_transform.y]
    ]
}
