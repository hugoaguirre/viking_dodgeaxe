extern crate ggez;

use ggez::conf;
use ggez::event;
use ggez::timer;
use ggez::event::MouseButton;
use ggez::{Context, GameResult};
use ggez::graphics;

use assets::Assets;
use arena::Arena;
use animation::ArenaAnimation;
use utils::{map_animation, draw_map};

use std::time::Duration;

mod assets;
mod arena;
mod animation;
mod utils;

struct MainState {
    /* This main state goes inside the Context, it has to have all
     * the globals needed to run the game */
    assets: Assets,
    arena: Arena,
}

impl MainState {

    fn new(ctx: &mut Context) -> GameResult<MainState> {
        println!("Created a new MainState struct!");
        let assets = Assets::new(ctx)?;
        let arena = Arena::new(map_animation(ctx)?);

        let ms = MainState {
            assets: assets,
            arena: arena,
        };

        Ok((ms))
    }
}

impl event::EventHandler for MainState {

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        draw_map(ctx, &mut self.arena.animation)?;
        graphics::present(ctx);
        timer::sleep(Duration::from_secs(0));
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> {
        Ok(())
    }

    fn key_down_event(&mut self,
                      keycode: event::Keycode, 
                      _keymod: event::Mod, 
                      _repeat: bool) {

        if keycode == event::Keycode::A {
            println!("A key pressed");
        }
    }

    fn mouse_button_down_event(&mut self, _button: MouseButton, _x: i32, _y: i32) {
        println!("Mouse pressed @ {}, {}", _x, _y);
    }
}

pub fn main()
{
    let mut c = conf::Conf::new();
    c.window_title = "Vikings: Dodgeaxe".to_string();
    c.window_width = 420;
    c.window_height = 210;

    let ctx = &mut Context::load_from_conf("vikings", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e)
    } else {
        println!("Exited cleanly")
    }
}
