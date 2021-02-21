use bevy::{input::system::exit_on_esc_system, prelude::*};
use std::{borrow::BorrowMut, collections::HashSet, hash::Hash, time::Duration};

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

#[derive(Clone)]
struct GameMovement {
    pressed_key: Option<KeyCode>,
    direction: Option<GameMovementDirection>,
    move_timer: Option<Timer>,
}

impl Default for GameMovement {
    fn default() -> Self {
        GameMovement {
            pressed_key: None,
            direction: None,
            move_timer: None,
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
        .add_event::<BoardMoveStart>()
        .add_event::<BoardMoveEnd>()
        .add_resource(ClearColor(Color::rgb_u8(187, 173, 160)))
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
        .add_system(game_movement_timer_ticker.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());

    commands
        .insert_resource(Materials::instantiate(materials));

    let game_board = GameBoard::new();

    commands
        .insert_resource(game_board);
}

fn block_spawner(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    materials: Res<Materials>,
    mut game_board: ResMut<GameBoard>,
) {
    for y in 0..ROWS_COUNT {
        for x in 0..COLS_COUNT {
            let pos = Position { x, y };

            let mut transform = Transform::from_translation(Vec3::default());

            transform_to_position(&pos, &mut transform);

            let sprite_bundle = SpriteBundle {
                material: materials.empty_color.clone(),
                sprite: Sprite::new(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                transform,
                ..Default::default()
            };

            commands.spawn(sprite_bundle)
                .with(pos)
                .with(BlockPlaceholder);
        }
    }

    blocks_spawner(
        commands,
        &asset_server,
        &materials,
        &mut game_board,
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
    game_board: &mut ResMut<GameBoard>,
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
        .with(Block);
    
        game_board.set_cell(block_position.x, block_position.y, Some(block_size.clone()));
    }
}

fn style_to_position(
    pos: &Position,
    style: &mut Style,
) {
    style.position.left = Val::Px(GAP + (pos.x as f32 * BLOCK_SIZE) + (pos.x as f32 * GAP));
    style.position.top = Val::Px(GAP + (pos.y as f32 * BLOCK_SIZE) + (pos.y as f32 * GAP));
}

fn move_style_to(
    pos: &Position,
    style: &mut Style,
    percent: f32,
) {
    let new_left = {
        let dest = GAP + (pos.x as f32 * BLOCK_SIZE) + (pos.x as f32 * GAP);
        let curr = match style.position.left {
            Val::Px(curr) => curr,
            _ => {
                panic!("expected position in pixels")
            }
        };

        Val::Px(curr + ((dest - curr) * percent))
    };

    let new_top = {
        let dest = GAP + (pos.y as f32 * BLOCK_SIZE) + (pos.y as f32 * GAP);
        let curr = match style.position.top {
            Val::Px(curr) => curr,
            _ => {
                panic!("expected position in pixels")
            }
        };

        Val::Px(curr + (dest - curr) * percent)
    };

    style.position.left = new_left;
    style.position.top = new_top;
}

fn transform_to_position(
    pos: &Position,
    transform: &mut Transform,
) {
    transform.translation = Vec3::new(
    GAP + (pos.x as f32 * BLOCK_SIZE) + (pos.x as f32 * GAP) - (WINDOW_WIDTH / 2.0) + (BLOCK_SIZE / 2.0),
    GAP + (pos.y as f32 * BLOCK_SIZE) + (pos.y as f32 * GAP) - (WINDOW_HEIGHT / 2.0) + (BLOCK_SIZE / 2.0),
    0.0
    );
}

fn position_translation(
    mut game_movement: ResMut<GameMovement>,
    mut q: Query<(&Position, &mut Style)>,
    mut board_moved_event: ResMut<Events<BoardMoveEnd>>,
) {
    if let Some(game_movement_timer) = game_movement.move_timer.to_owned() {
        if !game_movement_timer.finished() {
            for (pos, mut style) in q.iter_mut() {
                move_style_to(pos, &mut style, game_movement_timer.percent());
            }
        } else {
            board_moved_event.send(BoardMoveEnd);
            game_movement.move_timer = None;
        }
    }
}

fn game_movement_timer_ticker(
    time: Res<Time>,
    mut game_movement: ResMut<GameMovement>,
) {
    if let Some(game_movement_timer) = game_movement.move_timer.borrow_mut() {
        game_movement_timer.tick(time.delta_seconds());
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

    let (direction, pressed_key) = if keyboard_input.pressed(KeyCode::Up) {
        (Some(GameMovementDirection::Up), Some(KeyCode::Up))
    } else if keyboard_input.pressed(KeyCode::Right) {
        (Some(GameMovementDirection::Right), Some(KeyCode::Right))
    } else if keyboard_input.pressed(KeyCode::Down) {
        (Some(GameMovementDirection::Down), Some(KeyCode::Down))
    } else if keyboard_input.pressed(KeyCode::Left) {
        (Some(GameMovementDirection::Left), Some(KeyCode::Left))
    } else {
        (None, None)
    };

    game_movement.pressed_key = pressed_key;
    game_movement.direction = direction;

    if pressed_key.is_some() {
        game_movement.move_timer = Some(Timer::new(Duration::from_millis(200. as u64), false));
    }
}

fn movement(
    mut positions: Query<(Entity, &mut Position), With<Block>>,
    mut game_movement: ResMut<GameMovement>,
    mut game_board: ResMut<GameBoard>,
    mut board_moved_event: ResMut<Events<BoardMoveStart>>,
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

        board_moved_event.send(BoardMoveStart);

        game_board.move_board(direction);
    }
}

fn game_board_watcher(
    commands: &mut Commands,
    board_moved_evemts: Res<Events<BoardMoveEnd>>,
    asset_server: Res<AssetServer>,
    materials: Res<Materials>,
    block_sizes: Query<&BlockSize, With<Block>>,
    mut game_board: ResMut<GameBoard>,
    mut positions: Query<(Entity, &mut Position), With<Block>>,
    mut move_reader: Local<EventReader<BoardMoveEnd>>,
) {
    if move_reader.iter(&board_moved_evemts).next().is_none() {
        return;
    }

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
        blocks_spawner(commands, &asset_server, &materials, &mut game_board, blocks_to_spawn);
    }

    let (x, y) = game_board.rand_available_cell();

    blocks_spawner(commands, &asset_server, &materials, &mut game_board, vec!(
        (BlockSize::_2, Position::new(x, y)),
    ))
}
