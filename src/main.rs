use bevy::{input::system::exit_on_esc_system, prelude::*};
use std::{collections::HashSet, hash::Hash, time::Duration};

mod constants;
use constants::*;

mod game_board;
use game_board::*;

mod block;
use block::*;

mod materials;
use materials::*;

mod events;
use events::*;

struct MoveTimer(Timer);

#[derive(Clone, Copy)]
struct GameMovement {
    pressed_key: Option<KeyCode>,
    direction: Option<GameMovementDirection>,
}

impl Default for GameMovement {
    fn default() -> Self {
        GameMovement {
            pressed_key: None,
            direction: None,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Position {
    x: u8,
    y: u8,
}

impl Position {
    fn new(x: u8, y: u8) -> Self {
        Position {
            x,
            y,
        }
    }
}

fn main() {
    App::build()
        .add_event::<BoardMovedEvent>()
        .add_resource(WindowDescriptor {
            title: "2048".to_string(),
            height: WINDOW_HEIGHT,
            width: WINDOW_WIDTH,
            ..Default::default()
        })
        .add_resource(GameMovement::default())
        .add_resource(MoveTimer(Timer::new(
            Duration::from_millis(200. as u64),
            true,
        )))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage("spawn_initial_blocks", SystemStage::single(block_spawner.system()))
        .add_system(position_translation.system())
        .add_system(exit_on_esc_system.system())
        .add_system(input_movement.system())
        .add_system(movement.system())
        .add_system(game_board_watcher.system())
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
        let mut node_style = Style {
            size: Size {
                height: Val::Px(BLOCK_SIZE),
                width: Val::Px(BLOCK_SIZE),
            },
            align_content: AlignContent::Center,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            ..Default::default()
        };

        style_to_position(&block_position, &mut node_style);

        commands.spawn(NodeBundle {
            style: node_style,
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
        .with(block_size.clone())
        .with(BlockText);
    
        game_board.set_cell(block_position.x, block_position.y, Some(block_size.clone()));
    }
}

fn style_to_position(
    pos: &Position,
    style: &mut Style,
) {
    style.position.left = Val::Px(GAP + (pos.x as f32 * BLOCK_SIZE));
    style.position.top = Val::Px(GAP + (pos.y as f32 * BLOCK_SIZE));
}

fn position_translation(
    mut q: Query<(&Position, &mut Style), With<BlockText>>,
) {
    for (pos, mut style) in q.iter_mut() {
        style_to_position(pos, &mut style);
    }
}

fn input_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_movement: ResMut<GameMovement>
) {
    if game_movement.direction.is_some() {
        return;
    }
    
    if let Some(curr_pressed_key) = game_movement.pressed_key {
        if keyboard_input.just_released(curr_pressed_key) {
            game_movement.pressed_key = None;

            return;
        }

        return;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        game_movement.direction = Some(GameMovementDirection::Up);
        game_movement.pressed_key = Some(KeyCode::Up);
    } else if keyboard_input.pressed(KeyCode::Right) {
        game_movement.direction = Some(GameMovementDirection::Right);
        game_movement.pressed_key = Some(KeyCode::Right);
    } else if keyboard_input.pressed(KeyCode::Down) {
        game_movement.direction = Some(GameMovementDirection::Down);
        game_movement.pressed_key = Some(KeyCode::Down);
    } else if keyboard_input.pressed(KeyCode::Left) {
        game_movement.direction = Some(GameMovementDirection::Left);
        game_movement.pressed_key = Some(KeyCode::Left);
    }
}

fn movement(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    materials: Res<Materials>,
    block_sizes: Query<&BlockSize, With<BlockText>>,
    mut positions: Query<(Entity, &mut Position), With<BlockText>>,
    mut game_movement: ResMut<GameMovement>,
    mut game_board: ResMut<GameBoard>,
    mut board_moved_event: ResMut<Events<BoardMovedEvent>>,
) {
    if let Some(direction) = game_movement.direction {
        let mut any_block_moved = false;

        for (_, mut pos) in positions.iter_mut() {
            let available_steps = game_board.steps(pos.x, pos.y, direction);

            match direction {
                GameMovementDirection::Up => {
                    pos.y -= available_steps;
                },
                GameMovementDirection::Right => {
                    pos.x += available_steps;
                },
                GameMovementDirection::Down => {
                    pos.y += available_steps;
                },
                GameMovementDirection::Left => {
                    pos.x -= available_steps;
                },
            }

            if available_steps != 0 {
                any_block_moved = true;
            }
        }

        game_movement.direction = None;

        if !any_block_moved {
            return;
        }

        game_board.move_board(direction);
        // println!("game board moved: \n{}", game_board.pretty_string());

        let mut despawned_positions: HashSet<Position> = HashSet::new();
        let mut blocks_to_spawn: Vec<(BlockSize, Position)> = Vec::new();

        for (entity, pos) in positions.iter_mut() {
            let block_size = block_sizes.get(entity).unwrap();
            let game_board_block_size = game_board.get_cell(pos.x, pos.y).unwrap();

            // blocks merged and now have new size
            if block_size.ne(&game_board_block_size) {
                commands.despawn_recursive(entity);

                if despawned_positions.contains(&Position { x: pos.x, y: pos.y }) {
                    blocks_to_spawn.push((
                        game_board_block_size.clone(),
                        pos.clone(),
                    ));
                } else {
                    despawned_positions.insert(Position { x: pos.x, y: pos.y });
                }
            }
        }

        if blocks_to_spawn.len() > 0 {
            blocks_spawner(commands, &asset_server, &materials, game_board, blocks_to_spawn);
        }

        board_moved_event.send(BoardMovedEvent);
    }
}

fn game_board_watcher(
    commands: &mut Commands,
    board_moved_evemts: Res<Events<BoardMovedEvent>>,
    asset_server: Res<AssetServer>,
    materials: Res<Materials>,
    game_board: ResMut<GameBoard>,
    mut move_reader: Local<EventReader<BoardMovedEvent>>,
) {
    if move_reader.iter(&board_moved_evemts).next().is_none() {
        return;
    }

    let (x, y) = game_board.rand_available_cell();

    blocks_spawner(commands, &asset_server, &materials, game_board, vec!(
        (BlockSize::_2, Position::new(x, y)),
    ))
}
