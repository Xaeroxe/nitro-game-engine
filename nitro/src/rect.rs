use Vector;
use sdl2::rect::Rect as SdlRect;

pub struct Rect {
    pub xy: Vector,
    pub size: Vector,
}

impl Rect {
    fn new(x: f32, y: f32, width: f32, height: f32) -> Rect {
        Rect {
            xy: Vector::new(x, y),
            size: Vector::new(width, height),
        }
    }

    fn x(&self) -> f32 {
        self.xy.x
    }

    fn y(&self) -> f32 {
        self.xy.y
    }

    fn width(&self) -> f32 {
        self.size.x
    }

    fn height(&self) -> f32 {
        self.size.y
    }
}

impl From<SdlRect> for Rect {
    fn from(sdl_rect: SdlRect) -> Rect {
        Rect::new(sdl_rect.x() as f32,
                  sdl_rect.y() as f32,
                  sdl_rect.width() as f32,
                  sdl_rect.height() as f32)
    }
}

impl From<Rect> for SdlRect {
    fn from(rect: Rect) -> SdlRect {
        SdlRect::new(rect.x() as i32,
                     rect.y() as i32,
                     rect.width() as u32,
                     rect.height() as u32)
    }
}
