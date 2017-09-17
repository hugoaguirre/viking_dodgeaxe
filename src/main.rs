/* Force documentation */
#![deny(missing_docs)]

//! A vikings dodgeball game with axes a la Rust

extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use glutin_window::GlutinWindow;
use piston::input::RenderEvent;
use piston::event_loop:: {Events, EventSettings, EventLoop};
use opengl_graphics::{OpenGL, GlGraphics};
use graphics::{clear};
use drawing::color::BLACK;

mod drawing;

fn main() {
    let opengl = OpenGL::V3_2;

  /* Tell window backend which OpenGL version to use */
    let settings = WindowSettings::new("Vikings Dodgeaxe", [255, 255])
        .opengl(opengl)
        .exit_on_esc(true);

    let mut window: GlutinWindow = settings.build()
        .expect("Could not create main window");

    let mut events = Events::new(EventSettings::new());
    let mut gl = GlGraphics::new(opengl);

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
          /* Get rectangular area inside the frame
           * buffer and draw on it */
          gl.draw(args.viewport(), |c, g| {
            clear(BLACK, g);
          });
        }
    }
}

