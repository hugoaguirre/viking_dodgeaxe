use ggez::graphics;
use ggez::{Context, GameResult};

pub struct Assets {
    pub font: graphics::Font,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let font = graphics::Font::new(ctx, "/RobotoRegular.ttf", 10)?;

        let s = Assets {font: font};
        Ok(s)
    }
}
