use graphics::math::Matrix2d;

#[derive(Copy, Clone)]
pub struct Transform {
    transform : Matrix2d,
}

impl Transform {
    pub fn new() -> Transform {
        // zero format for Matrix2d. Reverse engineered from Piston.
        // x = 0, y = 0, rotation = 0
        Transform{transform:[[1.0, 0.0, 0.0],[0.0, 1.0, 0.0]]}
    }

    pub fn from_x_y_rotation(x : f64, y : f64, rotation : f64) -> Transform {
        let mut value = Transform::new();
        value.set_x(x);
        value.set_y(y);
        value.set_rotation(rotation);
        value
    }

    pub fn set_x(&mut self, x : f64) {
        self.transform[0][2] = x;
    }

    pub fn get_x(&self) -> f64 {
        self.transform[0][2]
    }

    pub fn add_x(&mut self, delta_x : f64) {
        let x = self.get_x();
        self.set_x(x + delta_x);
    }

    pub fn set_y(&mut self, y : f64) {
        self.transform[1][2] = y;
    }

    pub fn get_y(&self) -> f64 {
        self.transform[1][2]
    }

    pub fn add_y(&mut self, delta_y : f64) {
        let y = self.get_y();
        self.set_y(y + delta_y);
    }

    pub fn set_rotation(&mut self, rot : f64) {
        let c = rot.cos();
        let s = rot.sin();
        self.transform[0][0] = c;
        self.transform[0][1] = -s;
        self.transform[1][0] = s;
        self.transform[1][1] = c;
    }

    pub fn get_rotation(&self) -> f64 {
        self.transform[1][0].atan2(self.transform[0][0])
    }

    pub fn add_rotation(&mut self, delta_rotation : f64) {
        let rot = self.get_rotation();
        self.set_rotation(rot + delta_rotation);
    }
}

pub fn get_raw(nitro_transform : &Transform) -> Matrix2d {
    nitro_transform.transform
}
