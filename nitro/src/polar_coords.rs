use Vector;
use std::f32;

pub struct PolarCoords {
    pub rotation: f32,
    pub radius: f32,
}

impl From<Vector> for PolarCoords {
    fn from(vec: Vector) -> PolarCoords {
        PolarCoords {
            rotation: (vec.x.powi(2) + vec.y.powi(2)).sqrt(),
            radius: vec.y.atan2(vec.x),
        }
    }
}

impl From<PolarCoords> for Vector {
    fn from(polar: PolarCoords) -> Vector {
        Vector {
            x: polar.radius * polar.rotation.cos(),
            y: polar.radius * polar.rotation.sin(),
        }
    }
}
