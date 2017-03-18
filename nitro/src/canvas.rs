use graphics_private::texture;
use graphics_private::texture::Texture;
use rect::Rect;
use sdl2::rect::Rect as SdlRect;
use sdl2::rect::Point;
use math::Vector;
use sdl2::render::Renderer;

pub struct Canvas<'a> {
    renderer: &'a mut Renderer<'static>,
}

impl<'a> Canvas<'a> {
    pub fn new(renderer: &'a mut Renderer<'static>) -> Canvas<'a> {
        Canvas { renderer: renderer }
    }

    pub fn draw_texture(&mut self,
                        texture: &Texture,
                        src: Option<Rect>,
                        dst: Option<Rect>,
                        angle: f64,
                        center: Option<Vector>,
                        flip_horizontal: bool,
                        flip_vertical: bool)
                        -> Result<(), String> {
        let texture = texture::get_raw(texture);
        let sdl_src;
        let sdl_dst;
        let sdl_center;
        if let Some(rect) = src {
            sdl_src = Some(SdlRect::from(rect));
        } else {
            sdl_src = None;
        }
        if let Some(rect) = dst {
            sdl_dst = Some(SdlRect::from(rect));
        } else {
            sdl_dst = None;
        }
        if let Some(vec) = center {
            sdl_center = Some(Point::new(vec.x as i32, vec.y as i32));
        } else {
            sdl_center = None;
        }
        self.renderer.copy_ex(&*texture,
                              sdl_src,
                              sdl_dst,
                              angle,
                              sdl_center,
                              flip_horizontal,
                              flip_vertical)
    }
}
