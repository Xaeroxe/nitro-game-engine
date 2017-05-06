use graphics::Texture;
use graphics::SpriteSheet;

/// Something which can be displayed on screen.  This can optionally be animated.
pub enum Sprite {
    /// A static image.  No animation is available.
    Texture(Texture),

    /// An animation given by a SpriteSheet.
    ///
    /// Animation is not done automatically by Nitro, advancing through and selecting
    /// frames is typically the responsiblity of a Component delegated to controlling
    /// animation.
    SpriteSheet(SpriteSheet),
}
