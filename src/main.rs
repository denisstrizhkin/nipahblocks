use anyhow::anyhow;
use bevy::{
    asset::LoadedFolder,
    image::ImageSampler,
    input::mouse::AccumulatedMouseMotion,
    pbr::wireframe::{WireframeConfig, WireframePlugin},
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
    texture_map: HashMap<String, Rect>,
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
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }),
                ..default()
            }),
            WireframePlugin,
        ))
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::LoadingAssets), load_assets)
        .add_systems(
            Update,
            loading_assets.run_if(in_state(GameState::LoadingAssets)),
        )
        .add_systems(OnExit(GameState::LoadingAssets), setup_resources)
        .add_systems(OnEnter(GameState::InGame), (spawn_player, setup))
        .add_systems(Update, move_player.run_if(in_state(GameState::InGame)))
        .insert_resource(WireframeConfig {
            global: true,
            default_color: Color::WHITE,
        })
        .run();
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        block_textures: asset_server.load_folder(BLOCK_TEXTURES_DIR),
    });
}

fn loading_assets(
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
                HashMap::<String, AssetId<Image>>::new(),
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
                map.insert(path.to_string(), id);

                info!("Loaded texture {path} into atlas at {id}");
                anyhow::Ok((map, builder))
            },
        )?;
    let (layout, sources, mut image) = layout_builder.build()?;
    let texture_map = texture_map
        .into_iter()
        .fold(HashMap::new(), |mut map, (key, id)| {
            let urect = sources.texture_rect(&layout, id).unwrap();
            let size = layout.size;
            let rect = Rect::new(
                urect.min.x as f32 / size.x as f32,
                urect.min.y as f32 / size.y as f32,
                urect.max.x as f32 / size.x as f32,
                urect.max.y as f32 / size.y as f32,
            );
            map.insert(key, rect);
            map
        });
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
    game_resources: Res<GameResources>,
) {
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Block::new(
            game_resources.texture_map["coral_block_pore_bleached.png"],
            game_resources.texture_map["grass_side.png"],
            game_resources.texture_map["grass_side.png"],
            game_resources.texture_map["grass_side.png"],
            game_resources.texture_map["grass_top.png"],
            game_resources.texture_map["dirt.png"],
        ))),
        MeshMaterial3d(game_resources.material.clone()),
    ));
    info!("Rendered a cube");
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Block::new(
            game_resources.texture_map["grass_side.png"],
            game_resources.texture_map["grass_side.png"],
            game_resources.texture_map["grass_side.png"],
            game_resources.texture_map["grass_side.png"],
            game_resources.texture_map["grass_top.png"],
            game_resources.texture_map["dirt.png"],
        ))),
        MeshMaterial3d(game_resources.material.clone()),
        Transform::from_xyz(0.0, -2.0, 0.0),
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

// fn update_hud(player_q: Query<&Transform, With<Player>>, mut text: Single<&mut Text, With<Hud>>) {
//     let Ok(transform) = player_q.get_single() else {
//         return;
//     };
//     let (x, y, z) = transform.translation.into();
//     ***text = format!("position: {x:0.2}, {y:0.2}, {z:0.2}");
// }

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
