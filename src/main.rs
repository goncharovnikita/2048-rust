use bevy::{input::system::exit_on_esc_system, prelude::*};

const BLOCK_SIZE: f32 = 40.0;
const ROWS_COUNT: u8 = 4;
const COLS_COUNT: u8 = 4;
const GAP: f32 = 200.0;
const WINDOW_HEIGHT: f32 = BLOCK_SIZE * ROWS_COUNT as f32 + GAP * 2.0;
const WINDOW_WIDTH: f32 = BLOCK_SIZE * COLS_COUNT as f32 + GAP * 2.0;

#[derive(Clone)]
struct Materials {
    _2_color: Handle<ColorMaterial>,
    _4_color: Handle<ColorMaterial>,
    _8_color: Handle<ColorMaterial>,
    _16_color: Handle<ColorMaterial>,
    _32_color: Handle<ColorMaterial>,
    _64_color: Handle<ColorMaterial>,
    _128_color: Handle<ColorMaterial>,
    _256_color: Handle<ColorMaterial>,
    _512_color: Handle<ColorMaterial>,
    _1024_color: Handle<ColorMaterial>,
    _2048_color: Handle<ColorMaterial>,
    empty_color: Handle<ColorMaterial>,
    debug_color: Handle<ColorMaterial>,
    transparent_color: Handle<ColorMaterial>,
}

impl Materials {
    fn from_block_size(materials: &Materials, block_size: BlockSize) -> &Handle<ColorMaterial> {
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

#[derive(Clone, Copy)]
enum GameMovementDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy)]
struct GameMovement {
    direction: Option<GameMovementDirection>,
}

impl Default for GameMovement {
    fn default() -> Self {
        GameMovement {
            direction: None,
        }
    }
}

#[derive(Clone)]
enum BlockSize {
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
    fn to_string(&self) -> String {
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
}

struct Block;
struct BlockText;

#[derive(Clone, PartialEq)]
struct Position {
    x: u8,
    y: u8,
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "2048".to_string(),
            height: WINDOW_HEIGHT,
            width: WINDOW_WIDTH,
            ..Default::default()
        })
        .add_resource(GameMovement::default())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage("spawn_initial_blocks", SystemStage::single(block_spawner.system()))
        .add_system(position_translation.system())
        .add_system(exit_on_esc_system.system())
        .add_system(input_movement.system())
        .add_system(movement.system())
        // .add_system(debug_position.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());

    commands
        .insert_resource(Materials {
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
            empty_color: materials.add(Color::rgba(238.0 / 255.0, 228.0 / 255.0, 218.0 / 255.0, 0.35).into()),
            debug_color: materials.add(Color::rgb_u8(220, 20, 60).into()),
            transparent_color: materials.add(Color::rgba_u8(0, 0, 0, 0).into()),
        });
}

fn block_spawner(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    materials: Res<Materials>,
) {
    spawn_block(
        commands,
        &asset_server,
        &materials,
        BlockSize::_2,
        Position {
            x: 0,
            y: 0,
        },
    );
}

fn spawn_block(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &Res<Materials>,
    block_size: BlockSize,
    block_position: Position,
) {
    let font = asset_server.load("OpenSans-Regular.ttf");

    commands
        .spawn(SpriteBundle {
            sprite: Sprite::new(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
            material: Materials::from_block_size(&materials, block_size.clone()).clone(),
            ..Default::default()
        })
        .with(Block)
        .with(block_position.clone());

    commands.spawn(NodeBundle {
        style: Style {
            size: Size {
                height: Val::Px(BLOCK_SIZE),
                width: Val::Px(BLOCK_SIZE),
            },
            align_content: AlignContent::Center,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        material: materials.transparent_color.clone(),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn( TextBundle {
            text: Text {
                value: block_size.to_string(),
                font: font.clone(),
                style: TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    alignment: TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        vertical: VerticalAlign::Center,
                    },
                }
            },
            style: Style {
                align_self: AlignSelf::Center,
                ..Default::default()
            },
            ..Default::default()
        });
    })
    .with(block_position.clone())
    .with(BlockText);
}

fn position_translation(
    mut q: Query<(&Position, &mut Transform), With<Block>>,
    mut sq: Query<(&Position, &mut Style), With<BlockText>>,
) {
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            pos.x as f32 * BLOCK_SIZE - (COLS_COUNT / 2) as f32 * BLOCK_SIZE + BLOCK_SIZE as f32 / 2.0,
            pos.y as f32 * BLOCK_SIZE - (ROWS_COUNT / 2) as f32 * BLOCK_SIZE + BLOCK_SIZE as f32 / 2.0,
            0.0,
        );
    }

    for (pos, mut style) in sq.iter_mut() {
        style.position.left = Val::Px(GAP + (pos.x as f32 * BLOCK_SIZE));
        style.position.bottom = Val::Px(GAP + (pos.y as f32 * BLOCK_SIZE));
    }
}

fn input_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_movement: ResMut<GameMovement>
) {
    if keyboard_input.pressed(KeyCode::Up) {
        game_movement.direction = Some(GameMovementDirection::Up);
    } else if keyboard_input.pressed(KeyCode::Right) {
        game_movement.direction = Some(GameMovementDirection::Right);
    } else if keyboard_input.pressed(KeyCode::Down) {
        game_movement.direction = Some(GameMovementDirection::Down);
    } else if keyboard_input.pressed(KeyCode::Left) {
        game_movement.direction = Some(GameMovementDirection::Left);
    }
}

fn movement(
    mut game_movement: ResMut<GameMovement>,
    mut positions: Query<&mut Position>
) {
    if let Some(direction) = game_movement.direction {
        for (mut pos) in positions.iter_mut() {
            match direction {
                GameMovementDirection::Up => {
                    pos.y = ROWS_COUNT - 1;
                },
                GameMovementDirection::Right => {
                    pos.x = COLS_COUNT - 1;
                },
                GameMovementDirection::Down => {
                    pos.y = 0;
                },
                GameMovementDirection::Left => {
                    pos.x = 0;
                },
            }
        }

        game_movement.direction = None;
    }
}
