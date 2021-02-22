pub const BLOCK_SIZE: f32 = 40.0;
pub const ROWS_COUNT: u8 = 4;
pub const COLS_COUNT: u8 = 4;
pub const GAP: f32 = 20.0;
pub const WINDOW_HEIGHT: f32 = BLOCK_SIZE * ROWS_COUNT as f32 + GAP * 2.0 + GAP * (ROWS_COUNT - 1) as f32;
pub const WINDOW_WIDTH: f32 = BLOCK_SIZE * COLS_COUNT as f32 + GAP * 2.0 + GAP * (COLS_COUNT - 1) as f32;
pub const BLOCK_SPAWN_ANIMATION_DURATION_MILLIS: u64 = 200;

#[derive(Clone, Copy, Debug)]
pub enum GameMovementDirection {
    Up,
    Right,
    Down,
    Left,
}
