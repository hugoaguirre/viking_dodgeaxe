use ggez::event;

#[derive(Debug)]
pub struct Controls {
    pub up: event::Keycode,
    pub down: event::Keycode,
    pub left: event::Keycode,
    pub right: event::Keycode,
    pub throw: event::MouseButton,
}
