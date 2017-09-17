//! Rumble arena
use drawing::color::BLACK;

mod drawing;

pub struct Arena {
    //! All info related to Arena
    pub bg_color: [f64; 4];
}

impl Arena {
    /// Creates a new Arena
    pub fn new() -> Arena {
        Arena {
            bg_color: BLACK;
        }
    }
}
