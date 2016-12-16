use graphics::math::Matrix2d;

#[derive(Copy, Clone)]
pub struct Transform {
    //Used to store results of rotation manipulation.
    //When the raw value is requested this will be integrated into it before returning.
    rotation : f64,
    transform :Matrix2d,
}

impl Transform {
    pub fn new() -> Transform {
        // zero format for Matrix2d. Reverse engineered from Piston.
        // x = 0, y = 0, rotation = 0
        Transform{transform:[[1.0, 0.0, 0.0],[0.0, 1.0, 0.0]],rotation:0.0}
    }

    pub fn from_x_y_rotation(x : f64, y : f64, rotation : f64) -> Transform {
        let mut value = Transform::new();
        *value.x() = x;
        *value.y() = y;
        *value.rotation() = rotation;
        value
    }

    pub fn x(&mut self) -> &mut f64 {
        &mut self.transform[0][2]
    }

    pub fn y(&mut self) -> &mut f64 {
        &mut self.transform[1][2]
    }

    pub fn rotation(&mut self) -> &mut f64 {
        &mut self.rotation
    }

    fn set_rotation(&mut self, rot : f64) {
        let c = rot.cos();
        let s = rot.sin();
        self.transform[0][0] = c;
        self.transform[0][1] = -s;
        self.transform[1][0] = s;
        self.transform[1][1] = c;
    }
}

//This must be mut in order to apply the rotation.
pub fn get_raw(nitro_transform : &mut Transform) -> Matrix2d {
    //Integrate into the raw any rotation changes that might have happened.
    let rot = nitro_transform.rotation;
    nitro_transform.set_rotation(rot);
    nitro_transform.transform
}
