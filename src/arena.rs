//! Rumble arena

use piston_window::{clear, Button, Context, Input, PistonWindow};
use opengl_graphics::GlGraphics;

use drawing::color::BLACK;

pub struct Arena {
    /// All info related to Arena
    pub bg_color: [f32; 4],
//    player: player::Player,
}

impl Arena {
    /// Creates a new Arena
    pub fn new() -> Self {
        Arena {
            bg_color: BLACK,
//            player: player::Player::new(),
        }
    }

    pub fn run(&mut self, window: &mut PistonWindow, opengl: &mut GlGraphics) {
        while let Some(event) = window.next() {
            match event {
                Input::Render(args) => {
                    opengl.draw(args.viewport(), |context, graphics| {
                        self.draw(context, graphics)
                    });
                }

                Input::Press(Button::Keyboard(key)) => match key {
                    _ => println!("press key")
                },

                _ => {}
            }
        }
    }

    /// Draws all current live objects onto the screen
    fn draw(&self, context: Context, graphics: &mut GlGraphics) {
        clear(self.bg_color, graphics);
        // Delegate entities drawing
//        self.player.draw(context, graphics);
    }
}
