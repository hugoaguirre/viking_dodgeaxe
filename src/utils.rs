use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::timer;

use std::time::Duration;

use arena::Arena;
use assets::Assets;
use animation::ArenaAnimation;

pub fn map_animation(ctx: &mut Context) -> GameResult<ArenaAnimation> {
    let arena_anim = ArenaAnimation {
        standby: [
            sprite(ctx, "/map1.png")?,
            sprite(ctx, "/map2.png")?,
            sprite(ctx, "/map3.png")?,
            sprite(ctx, "/map4.png")?,
            ],
    };

    Ok(arena_anim)
}

fn sprite(ctx: &mut Context, s: &str) -> GameResult<graphics::Image> {
    let mut sprite = graphics::Image::new(ctx, s)?;
    sprite.set_filter(graphics::FilterMode::Nearest);
    Ok(sprite)
}

pub fn draw_map(ctx: &mut Context, arena: &mut ArenaAnimation) -> GameResult<()> {
    for i in 0..arena.standby.len() {
      graphics::draw_ex(ctx, &arena.standby[i],
                        graphics::DrawParam {
                          dest: graphics::Point {
                              x: 210.,
                              y: 105.,
                          },
                          ..Default::default()
                        })?;
      graphics::present(ctx);
      timer::sleep(Duration::from_millis(110));
    }
    Ok(())
}
