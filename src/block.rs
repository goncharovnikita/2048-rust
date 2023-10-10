use bevy::prelude::Component;

#[derive(Component)]
pub struct Block;

#[derive(Component)]
pub struct BlockPlaceholder;

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum BlockSize {
    _2,
    _4,
    _8,
    _16,
    _32,
    _64,
    _128,
    _256,
    _512,
    _1024,
    _2048,
}

impl BlockSize {
    pub fn to_string(&self) -> String {
        match self {
            BlockSize::_2 => String::from("2"),
            BlockSize::_4 => String::from("4"),
            BlockSize::_8 => String::from("8"),
            BlockSize::_16 => String::from("16"),
            BlockSize::_32 => String::from("32"),
            BlockSize::_64 => String::from("64"),
            BlockSize::_128 => String::from("128"),
            BlockSize::_256 => String::from("256"),
            BlockSize::_512 => String::from("512"),
            BlockSize::_1024 => String::from("1024"),
            BlockSize::_2048 => String::from("2048"),
        }
    }

    pub fn next(&self) -> Option<Self> {
        match self {
            BlockSize::_2 => Some(BlockSize::_4),
            BlockSize::_4 => Some(BlockSize::_8),
            BlockSize::_8 => Some(BlockSize::_16),
            BlockSize::_16 => Some(BlockSize::_32),
            BlockSize::_32 => Some(BlockSize::_64),
            BlockSize::_64 => Some(BlockSize::_128),
            BlockSize::_128 => Some(BlockSize::_256),
            BlockSize::_256 => Some(BlockSize::_512),
            BlockSize::_512 => Some(BlockSize::_1024),
            BlockSize::_1024 => Some(BlockSize::_2048),
            BlockSize::_2048 => None,
        }
    }
}
