use math::Vector;
use std::f32;

/// Polar coordinates, can be converted to and from nitro::math::Vector.
///
/// These are useful when doing math that can be more easily described in terms of polar
/// coordinates, such as when describing orbital patterns.
///
/// Vector::from(PolarCoords) and PolarCoords::from(Vector) are the conversion functions.
pub struct PolarCoords {
    pub rotation: f32,
    pub radius: f32,
}

impl From<Vector> for PolarCoords {
    fn from(vec: Vector) -> PolarCoords {
        PolarCoords {
            radius: (vec.x.powi(2) + vec.y.powi(2)).sqrt(),
            rotation: vec.y.atan2(vec.x),
        }
    }
}

impl From<PolarCoords> for Vector {
    fn from(polar: PolarCoords) -> Vector {
        Vector::new(
            polar.radius * polar.rotation.cos(),
            polar.radius * polar.rotation.sin()
        )
    }
}
