use bevy::{input::system::exit_on_esc_system, prelude::*, transform};

const BLOCK_SIZE: f32 = 40.0;
const ROWS_COUNT: u8 = 4;
const COLS_COUNT: u8 = 4;
const GAP: f32 = 200.0;
const WINDOW_HEIGHT: f32 = BLOCK_SIZE * ROWS_COUNT as f32 + GAP * 2.0;
const WINDOW_WIDTH: f32 = BLOCK_SIZE * COLS_COUNT as f32 + GAP * 2.0;

struct Materials {
    two_color: Handle<ColorMaterial>,
    four_color: Handle<ColorMaterial>,
    eight_color: Handle<ColorMaterial>,
    sixteen_color: Handle<ColorMaterial>,
    thirty_two_color: Handle<ColorMaterial>,
    sixty_four_color: Handle<ColorMaterial>,
    one_two_eight_color: Handle<ColorMaterial>,
    two_five_six_color: Handle<ColorMaterial>,
    five_one_two_color: Handle<ColorMaterial>,
    one_zero_two_four_color: Handle<ColorMaterial>,
    two_zero_four_eight_color: Handle<ColorMaterial>,
    empty_color: Handle<ColorMaterial>,
    debug_color: Handle<ColorMaterial>,
    transparent_color: Handle<ColorMaterial>,
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
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage("spawn_initial_blocks", SystemStage::single(block_spawner.system()))
        .add_system(position_translation.system())
        .add_system(exit_on_esc_system.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());

    commands
        .insert_resource(Materials {
            two_color: materials.add(Color::rgb_u8(238, 228, 218).into()),
            four_color: materials.add(Color::rgb_u8(237, 224, 200).into()),
            eight_color: materials.add(Color::rgb_u8(242, 177, 121).into()),
            sixteen_color: materials.add(Color::rgb_u8(245, 149, 99).into()),
            thirty_two_color: materials.add(Color::rgb_u8(246, 124, 95).into()),
            sixty_four_color: materials.add(Color::rgb_u8(246, 94, 59).into()),
            one_two_eight_color: materials.add(Color::rgb_u8(237, 207, 114).into()),
            two_five_six_color: materials.add(Color::rgb_u8(237, 204, 97).into()),
            five_one_two_color: materials.add(Color::rgb_u8(237, 200, 80).into()),
            one_zero_two_four_color: materials.add(Color::rgb_u8(237, 197, 63).into()),
            two_zero_four_eight_color: materials.add(Color::rgb_u8(237, 194, 46).into()),
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
    let font = asset_server.load("OpenSans-Regular.ttf");

    commands
        .spawn(SpriteBundle {
            sprite: Sprite::new(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
            material: materials.two_color.clone(),
            ..Default::default()
        })
        .with(Block)
        .with(Position {
            x: 0,
            y: 0,
        });

    commands.spawn(NodeBundle {
        style: Style {
            size: Size {
                height: Val::Px(BLOCK_SIZE),
                width: Val::Px(BLOCK_SIZE),
            },
            align_content: AlignContent::Center,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        material: materials.transparent_color.clone(),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn( TextBundle {
            text: Text {
                value: "2".to_string(),
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
    .with(Position {
        x: 0,
        y: 0,
    })
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
