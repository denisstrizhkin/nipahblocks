use anyhow::anyhow;
use bevy::{
    asset::LoadedFolder,
    image::ImageSampler,
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        settings::{RenderCreation, WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
};
use rand::RngCore;
use std::{collections::HashMap, fs};

mod block;
mod block_registry;
mod chunk;
mod player;

use block::Block;
use block_registry::BlockInfoRegistry;
use chunk::Chunk;
use player::PlayerPlugin;

const BLOCK_INFO_REGISTRY: &str = "assets/block_registry.json";
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
    block_registry_json: String,
}

#[derive(Debug, Resource)]
struct GameResources {
    texture_atlas: Handle<Image>,
    texture_map: HashMap<String, Rect>,
    material: Handle<StandardMaterial>,
    blocks: Vec<Block>,
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
            PlayerPlugin,
            WireframePlugin,
        ))
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::LoadingAssets), load_assets)
        .add_systems(
            Update,
            loading_assets.run_if(in_state(GameState::LoadingAssets)),
        )
        .add_systems(OnExit(GameState::LoadingAssets), setup_resources)
        .add_systems(OnEnter(GameState::InGame), setup)
        .insert_resource(WireframeConfig {
            global: true,
            default_color: Color::WHITE,
        })
        .run();
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) -> Result {
    let block_textures = asset_server.load_folder(BLOCK_TEXTURES_DIR);
    let block_registry_json = fs::read_to_string(BLOCK_INFO_REGISTRY)?;
    commands.insert_resource(GameAssets {
        block_textures,
        block_registry_json,
    });
    Ok(())
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
    let block_info_registry =
        serde_json::from_str::<BlockInfoRegistry>(&game_assets.block_registry_json)?;
    let blocks = block_info_registry
        .blocks
        .into_iter()
        .map(|block_info| {
            Block::new(
                texture_map[&block_info.front],
                texture_map[&block_info.back],
                texture_map[&block_info.left],
                texture_map[&block_info.right],
                texture_map[&block_info.top],
                texture_map[&block_info.bottom],
            )
        })
        .collect();
    commands.insert_resource(GameResources {
        texture_atlas,
        texture_map,
        material,
        blocks,
    });
    Ok(())
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    game_resources: Res<GameResources>,
) {
    // chunk
    let mut chunk1 = Chunk::default();
    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                chunk1.set_at(UVec3::new(x, y, z), Some(&game_resources.blocks[2]));
            }
        }
    }
    commands.spawn((
        Mesh3d(meshes.add(chunk1)),
        MeshMaterial3d(game_resources.material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    let mut chunk1 = Chunk::default();
    let mut rng = rand::thread_rng();
    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                let i = (rng.next_u32() % 4) as usize;
                if i != 3 {
                    chunk1.set_at(UVec3::new(x, y, z), Some(&game_resources.blocks[i]));
                }
            }
        }
    }
    commands.spawn((
        Mesh3d(meshes.add(chunk1)),
        MeshMaterial3d(game_resources.material.clone()),
        Transform::from_xyz(32.0, 0.0, 0.0),
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

fn generate_chunks(blocks: &[Block]) -> Vec<Chunk> {
    let height = 256 / 16;
    let width = 256 / 16;
    todo!()
}

// fn update_hud(player_q: Query<&Transform, With<Player>>, mut text: Single<&mut Text, With<Hud>>) {
//     let Ok(transform) = player_q.get_single() else {
//         return;
//     };
//     let (x, y, z) = transform.translation.into();
//     ***text = format!("position: {x:0.2}, {y:0.2}, {z:0.2}");
// }
