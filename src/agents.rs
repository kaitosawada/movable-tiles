pub mod player;
pub mod ant;

#[derive(Clone, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub old_x: i32,
    pub old_y: i32,
}

pub const SPEED: i32 = 1;