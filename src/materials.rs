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
    pub text_primary_color: Color,
    pub text_inverted_color: Color,
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

    pub fn instantiate(mut materials: ResMut<Assets<ColorMaterial>>) -> Self {
        Materials {
            _2_color: materials.add(Color::rgb_u8(238, 228, 218).into()),
            _4_color: materials.add(Color::rgb_u8(237, 224, 200).into()),
            _8_color: materials.add(Color::rgb_u8(242, 177, 121).into()),
            _16_color: materials.add(Color::rgb_u8(245, 149, 99).into()),
            _32_color: materials.add(Color::rgb_u8(246, 124, 95).into()),
            _64_color: materials.add(Color::rgb_u8(246, 94, 59).into()),
            _128_color: materials.add(Color::rgb_u8(237, 207, 114).into()),
            _256_color: materials.add(Color::rgb_u8(237, 204, 97).into()),
            _512_color: materials.add(Color::rgb_u8(237, 200, 80).into()),
            _1024_color: materials.add(Color::rgb_u8(237, 197, 63).into()),
            _2048_color: materials.add(Color::rgb_u8(237, 194, 46).into()),
            text_inverted_color: Color::WHITE,
            text_primary_color: Color::rgb_u8(119, 110, 101),
            empty_color: materials.add(Color::rgba(238.0 / 255.0, 228.0 / 255.0, 218.0 / 255.0, 0.35).into()),
            debug_color: materials.add(Color::rgb_u8(220, 20, 60).into()),
            transparent_color: materials.add(Color::rgba_u8(0, 0, 0, 0).into()),
        }
    }
}
