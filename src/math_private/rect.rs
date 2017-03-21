use math::Vector;

#[derive(Debug, Copy, PartialEq, Clone)]
pub struct Rect {
    pub xy: Vector,
    pub size: Vector,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Rect {
        Rect {
            xy: Vector::new(x, y),
            size: Vector::new(width, height),
        }
    }

    pub fn x(&self) -> f32 {
        self.xy.x
    }

    pub fn y(&self) -> f32 {
        self.xy.y
    }

    pub fn width(&self) -> f32 {
        self.size.x
    }

    pub fn height(&self) -> f32 {
        self.size.y
    }
}

