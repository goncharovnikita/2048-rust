use std::time::Duration;
use bevy::prelude::*;

use crate::constants::*;

pub struct AnimateBlockTimer(pub Timer);

impl Default for AnimateBlockTimer {
    fn default() -> Self {
        AnimateBlockTimer(
            Timer::new(Duration::from_millis(BLOCK_SPAWN_ANIMATION_DURATION_MILLIS), false)
        )
    }
}
