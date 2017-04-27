use math::Transform;

/// Describes a viewpoint into the world.
pub struct Camera {
    pub transform: Transform,
    pub zoom: f32,
}
