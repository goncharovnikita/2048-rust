use bevy::{input::system::exit_on_esc_system, prelude::*};

mod constants;
use constants::*;

mod game_board;
use game_board::*;

mod block;
use block::*;

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

    let game_board = GameBoard::new([[None; COLS_COUNT as usize]; ROWS_COUNT as usize]);

    commands
        .insert_resource(game_board);
}

fn block_spawner(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    materials: Res<Materials>,
    game_board: ResMut<GameBoard>,
) {
    blocks_spawner(
        commands,
        &asset_server,
        &materials,
        game_board,
        vec![
            (
                BlockSize::_2,
                Position {
                    x: 0,
                    y: 0,
                },
            ),
            (
                BlockSize::_4,
                Position {
                    x: 2,
                    y: 3,
                },
            ),
            (
                BlockSize::_2,
                Position {
                    x: 1,
                    y: 3,
                },
            ),
        ]
    );
}

fn blocks_spawner(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &Res<Materials>,
    mut game_board: ResMut<GameBoard>,
    blocks: Vec<(BlockSize, Position)>,
) {
    let font = asset_server.load("OpenSans-Regular.ttf");

    for (block_size, block_position) in blocks.iter() {
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
            material: Materials::from_block_size(&materials, block_size.clone()).clone(),
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
    
        game_board.set_cell(block_position.x, block_position.y, Some(block_size.clone()));
    }

    println!("initial game board: {}", game_board.pretty_string());
}

fn position_translation(
    mut q: Query<(&Position, &mut Style), With<BlockText>>,
) {
    for (pos, mut style) in q.iter_mut() {
        style.position.left = Val::Px(GAP + (pos.x as f32 * BLOCK_SIZE));
        style.position.top = Val::Px(GAP + (pos.y as f32 * BLOCK_SIZE));
    }
}

fn input_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_movement: ResMut<GameMovement>
) {
    if game_movement.direction.is_some() {
        return;
    }

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
    mut positions: Query<&mut Position, With<BlockText>>,
    mut game_board: ResMut<GameBoard>,
) {
    if let Some(direction) = game_movement.direction {
        for mut pos in positions.iter_mut() {
            match direction {
                GameMovementDirection::Up => {
                    let available_steps = game_board.steps(pos.x, pos.y, GameMovementDirection::Up);

                    pos.y -= available_steps;
                },
                GameMovementDirection::Right => {
                    let available_steps = game_board.steps(pos.x, pos.y, GameMovementDirection::Right);

                    pos.x += available_steps;
                },
                GameMovementDirection::Down => {
                    let available_steps = game_board.steps(pos.x, pos.y, GameMovementDirection::Down);
                    
                    pos.y += available_steps;
                },
                GameMovementDirection::Left => {
                    let available_steps = game_board.steps(pos.x, pos.y, GameMovementDirection::Left);
                    
                    pos.x -= available_steps;
                },
            }
        }

        game_movement.direction = None;
        game_board.move_board(direction);
        println!("game board moved: \n{}", game_board.pretty_string());
    }
}
