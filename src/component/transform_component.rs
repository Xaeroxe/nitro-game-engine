use specs::Component;
use math::Transform;

#[derive(Debug, Copy, Clone)]
pub struct TransformComponent {
    pub transform: Transform
}

impl Component for TransformComponent {}