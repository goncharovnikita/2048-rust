use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;

pub fn animate_block_spawned(
    time: Res<Time>,
    mut blocks: Query<(&mut Style, &mut AnimateBlockTimer)>
) {
    for (mut style, mut block_timer) in blocks.iter_mut() {
        block_timer.0.tick(time.delta_seconds());

        animate_block(&mut style, block_timer.0.percent());
    }
}

fn animate_block(
    style: &mut Mut<Style>,
    percent: f32
) {
    let curr_height = match style.size.height {
        Val::Px(height) => height,
        _ => unreachable!()
    };

    let curr_width = match style.size.width {
        Val::Px(width) => width,
        _ => unreachable!()
    };

    let curr_top = match style.margin.top {
        Val::Px(top) => top,
        _ => 0.0,
    };

    let curr_left = match style.margin.left {
        Val::Px(left) => left,
        _ => 0.0,
    };

    if percent >= 0.5 {
        style.size.height = Val::Px(curr_height - 2.0);
        style.size.width = Val::Px(curr_width - 2.0);
        style.margin.top = Val::Px(curr_top + 1.0);
        style.margin.left = Val::Px(curr_left + 1.0);
    } else {
        style.size.height = Val::Px(curr_height + 2.0);
        style.size.width = Val::Px(curr_width + 2.0);
        style.margin.top = Val::Px(curr_top - 1.0);
        style.margin.left = Val::Px(curr_left - 1.0);
    }

    if percent == 1.0 {
        style.size.height = Val::Px(BLOCK_SIZE);
        style.size.width = Val::Px(BLOCK_SIZE);
        style.margin.top = Val::Px(0.0);
        style.margin.left = Val::Px(0.0);
    }
}
