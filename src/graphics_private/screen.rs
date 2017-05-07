use std::collections::HashMap;
use sdl2::render::WindowCanvas;

pub struct Screen {
    texture_cache: HashMap<String, Arc<SdlTexture>>,
    canvas: WindowCanvas,
    /// How many pixels = 1 game world unit.  Defaults to 100.
    pub screen_to_world_ratio: f32,

    /// Distance in pixels from screen at which an object won't be drawn.  Defaults to 750.
    pub cull_padding: u32,
}

impl Screen {
    pub fn new(canvas: WindowCanvas) -> Screen {
        Screen {
            canvas,
            texture_cache: HashMap::new(),
            screen_to_world_ratio: 100.0,
            cull_padding: 750,
        }
    }
}