use anyhow::anyhow;
use bevy::{
    asset::LoadedFolder,
    image::ImageSampler,
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    render::{
        settings::{RenderCreation, WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
};
use std::{collections::HashMap, f32::consts::FRAC_PI_2};

mod block;
use block::Block;

const BLOCK_TEXTURES_DIR: &str = "../assets/textures/blocks";
const TILEMAP_COLUMNS: u32 = 10;
const TILEMAP_ROWS: u32 = 16;
const TILE_WIDTH: u32 = 16;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
enum GameState {
    #[default]
    LoadingAssets,
    InGame,
}

#[derive(Debug, Resource)]
struct GameAssets {
    block_textures: Handle<LoadedFolder>,
}

#[derive(Debug, Resource)]
struct GameResources {
    texture_atlas: Handle<Image>,
    texture_map: HashMap<String, u32>,
    material: Handle<StandardMaterial>,
}

#[derive(Debug, Component)]
struct Player {
    movement_speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            movement_speed: 5.0,
        }
    }
}

#[derive(Debug, Component, Deref, DerefMut)]
struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.002, 0.002))
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                features: WgpuFeatures::POLYGON_MODE_LINE,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::LoadingAssets), load_assets)
        .add_systems(Update, loading.run_if(in_state(GameState::LoadingAssets)))
        .add_systems(OnEnter(GameState::InGame), setup_resources)
        .add_systems(Startup, (spawn_player, setup))
        .add_systems(Update, move_player)
        .run();
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        block_textures: asset_server.load_folder(BLOCK_TEXTURES_DIR),
    });
}

fn loading(
    mut next_state: ResMut<NextState<GameState>>,
    game_assets: Res<GameAssets>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    for event in events.read() {
        if event.is_loaded_with_dependencies(&game_assets.block_textures) {
            next_state.set(GameState::InGame);
        }
    }
}

fn setup_resources(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_assets: Res<GameAssets>,
    loaded_folders: Res<Assets<LoadedFolder>>,
) -> Result {
    let (texture_map, mut layout_builder) = loaded_folders
        .get(&game_assets.block_textures)
        .ok_or(anyhow!("Couldn't load block textures folder"))?
        .handles
        .iter()
        .try_fold(
            (
                HashMap::<String, u32>::new(),
                TextureAtlasBuilder::default(),
            ),
            |(mut map, mut builder), handle| {
                let id = handle.id().try_typed::<Image>()?;
                let path = handle
                    .path()
                    .and_then(|p| p.path().file_name())
                    .map(|n| n.to_string_lossy())
                    .ok_or(anyhow!("Failed to retrieve image's file name"))?;

                let texture = images
                    .get(id)
                    .ok_or(anyhow!("Failed to retrieve image: {path}"))?;

                builder.add_texture(Some(id), texture);
                if map.contains_key(path.as_ref()) {
                    return Err(anyhow!("Duplicate image: {path}"));
                }
                let index = map.len();
                map.insert(path.to_string(), index as u32);

                info!("Loaded texture {path} into atlas at {index}");
                anyhow::Ok((map, builder))
            },
        )?;
    let (_layout, _sources, mut image) = layout_builder.build()?;
    image.sampler = ImageSampler::nearest();
    let texture_atlas = images.add(image);
    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(texture_atlas.clone()),
        perceptual_roughness: 0.97,
        reflectance: 0.1,
        ..default()
    });
    commands.insert_resource(GameResources {
        texture_atlas,
        texture_map,
        material,
    });
    Ok(())
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Block::new(0, 0, 0, 0, 0, 0))),
        MeshMaterial3d(materials.add(Color::BLACK)),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn((
            Player::default(),
            CameraSensitivity::default(),
            Transform::from_xyz(2.0, 0.5, 2.0),
            Visibility::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 45.0_f32.to_radians(),
                    ..default()
                }),
            ));
        });
}

fn move_player(
    time: Res<Time>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<(&mut Transform, &Player, &CameraSensitivity), With<Player>>,
) {
    let Ok((mut transform, player, camera_sensitivity)) = player_q.get_single_mut() else {
        return;
    };
    let delta = accumulated_mouse_motion.delta;
    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
    let mut direction = Vec3::ZERO;
    if keyboard.pressed(KeyCode::ArrowUp) {
        direction += *transform.forward();
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        direction += *transform.back();
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        direction += *transform.left();
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        direction += *transform.right();
    }
    direction.y = 0.0;
    let movement = direction.normalize_or_zero() * player.movement_speed * time.delta_secs();
    transform.translation += movement;
}
