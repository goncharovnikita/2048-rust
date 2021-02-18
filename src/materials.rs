use bevy::prelude::*;

use crate::block::BlockSize;

#[derive(Clone)]
pub struct Materials {
    pub _2_color: Handle<ColorMaterial>,
    pub _4_color: Handle<ColorMaterial>,
    pub _8_color: Handle<ColorMaterial>,
    pub _16_color: Handle<ColorMaterial>,
    pub _32_color: Handle<ColorMaterial>,
    pub _64_color: Handle<ColorMaterial>,
    pub _128_color: Handle<ColorMaterial>,
    pub _256_color: Handle<ColorMaterial>,
    pub _512_color: Handle<ColorMaterial>,
    pub _1024_color: Handle<ColorMaterial>,
    pub _2048_color: Handle<ColorMaterial>,
    pub empty_color: Handle<ColorMaterial>,
    pub debug_color: Handle<ColorMaterial>,
    pub transparent_color: Handle<ColorMaterial>,
}

impl Materials {
    pub fn from_block_size(materials: &Materials, block_size: BlockSize) -> &Handle<ColorMaterial> {
        match block_size {
            BlockSize::_2 => &materials._2_color,
            BlockSize::_4 => &materials._4_color,
            BlockSize::_8 => &materials._8_color,
            BlockSize::_16 => &materials._16_color,
            BlockSize::_32 => &materials._32_color,
            BlockSize::_64 => &materials._64_color,
            BlockSize::_128 => &materials._128_color,
            BlockSize::_256 => &materials._256_color,
            BlockSize::_512 => &materials._512_color,
            BlockSize::_1024 => &materials._1024_color,
            BlockSize::_2048 => &materials._2048_color,
        }
    }
}
