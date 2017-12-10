use animation::ArenaAnimation;

#[derive(Debug)]
pub struct Arena {
    pub animation: ArenaAnimation,
}

impl Arena {
    pub fn new(animation: ArenaAnimation) -> Arena {
        Arena {
            animation: animation,
        }
    }
}
