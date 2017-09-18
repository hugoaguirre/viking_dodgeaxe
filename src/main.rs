/* Force documentation */
#![deny(missing_docs)]

//! A vikings dodgeball game with axes a la Rust

extern crate piston_window;
extern crate opengl_graphics;

use piston_window::{OpenGL, PistonWindow, WindowSettings};
use opengl_graphics::GlGraphics;

use arena::Arena;
mod drawing;
mod arena;

fn main() {
    let opengl = OpenGL::V3_2;

    /* Tell window backend which OpenGL version to use */
    let settings = WindowSettings::new("Vikings Dodgeaxe", [420, 210])
        .opengl(opengl)
        .exit_on_esc(true);

    let mut window: PistonWindow = settings.build()
        .expect("Could not create main window");
    let mut gl = GlGraphics::new(opengl);

    let mut arena = Arena::new();

    // main loop
    arena.run(&mut window, &mut gl);
}

