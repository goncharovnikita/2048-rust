use bevy::prelude::*;

use crate::block::BlockSize;

#[derive(Resource, Clone)]
pub struct Materials {
    pub _2_color: Color,
    pub _4_color: Color,
    pub _8_color: Color,
    pub _16_color: Color,
    pub _32_color: Color,
    pub _64_color: Color,
    pub _128_color: Color,
    pub _256_color: Color,
    pub _512_color: Color,
    pub _1024_color: Color,
    pub _2048_color: Color,
    pub empty_color: Color,
    pub debug_color: Color,
    pub text_primary_color: Color,
    pub text_inverted_color: Color,
    pub transparent_color: Color,
}

impl Materials {
    pub fn from_block_size(materials: &Materials, block_size: BlockSize) -> Color {
        match block_size {
            BlockSize::_2 => materials._2_color,
            BlockSize::_4 => materials._4_color,
            BlockSize::_8 => materials._8_color,
            BlockSize::_16 => materials._16_color,
            BlockSize::_32 => materials._32_color,
            BlockSize::_64 => materials._64_color,
            BlockSize::_128 => materials._128_color,
            BlockSize::_256 => materials._256_color,
            BlockSize::_512 => materials._512_color,
            BlockSize::_1024 => materials._1024_color,
            BlockSize::_2048 => materials._2048_color,
        }
    }

    pub fn should_use_inverted_color(block_size: &BlockSize) -> bool {
        match block_size {
            BlockSize::_8 => true,
            BlockSize::_16 => true,
            BlockSize::_32 => true,
            BlockSize::_64 => true,
            BlockSize::_512 => true,
            BlockSize::_1024 => true,
            BlockSize::_2048 => true,
            _ => false,
        }
    }

    pub fn font_scale(block_size: &BlockSize) -> f32 {
        match block_size {
            BlockSize::_1024 => 0.8,
            BlockSize::_2048 => 0.8,
            _ => 1.0,
        }
    }

    pub fn instantiate() -> Self {
        Materials {
            _2_color: Color::rgb_u8(238, 228, 218),
            _4_color: Color::rgb_u8(237, 224, 200),
            _8_color: Color::rgb_u8(242, 177, 121),
            _16_color: Color::rgb_u8(245, 149, 99),
            _32_color: Color::rgb_u8(246, 124, 95),
            _64_color: Color::rgb_u8(246, 94, 59),
            _128_color: Color::rgb_u8(237, 207, 114),
            _256_color: Color::rgb_u8(237, 204, 97),
            _512_color: Color::rgb_u8(237, 200, 80),
            _1024_color: Color::rgb_u8(237, 197, 63),
            _2048_color: Color::rgb_u8(237, 194, 46),
            text_inverted_color: Color::WHITE,
            text_primary_color: Color::rgb_u8(119, 110, 101),
            empty_color: Color::rgba(238.0 / 255.0, 228.0 / 255.0, 218.0 / 255.0, 0.35),
            debug_color: Color::rgb_u8(220, 20, 60),
            transparent_color: Color::rgba_u8(0, 0, 0, 0),
        }
    }
}
