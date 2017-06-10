use math::IntVector;
use sdl2::rect::Rect as SdlRect;

#[derive(Debug, Copy, PartialEq, Clone)]
pub struct IntRect {
    pub xy: IntVector,
    pub width: u32,
    pub height: u32,
}

impl IntRect {
    fn new(x: i32, y: i32, width: u32, height: u32) -> IntRect {
        IntRect {
            xy: IntVector::new(x, y),
            width: width,
            height: height,
        }
    }

    pub fn x(&self) -> i32 {
        self.xy.x
    }

    pub fn y(&self) -> i32 {
        self.xy.y
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl From<SdlRect> for IntRect {
    fn from(sdl_rect: SdlRect) -> IntRect {
        IntRect::new(sdl_rect.x(),
                     sdl_rect.y(),
                     sdl_rect.width(),
                     sdl_rect.height())
    }
}

impl From<IntRect> for SdlRect {
    fn from(int_rect: IntRect) -> SdlRect {
        SdlRect::new(int_rect.x(),
                     int_rect.y(),
                     int_rect.width(),
                     int_rect.height())
    }
}
