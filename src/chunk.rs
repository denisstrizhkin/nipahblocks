use bevy::{prelude::*, utils::hashbrown::HashMap};
use noise::NoiseFn;
use std::sync::Arc;

use crate::block::{Block, BLOCK_HALF_SIZE};
use crate::player::Player;
use crate::{GameResources, GameState};

const CHUNK_SIZE: u32 = 16;
const CHUNK_LEN: u32 = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;
const CHUNK_OFFSET: f32 = -(CHUNK_SIZE as f32 / 2.0) + BLOCK_HALF_SIZE;

const SEED: u32 = 123456;

const DRAW_DISTANCE: u32 = 2;

pub struct ChunksPlugin;

impl Plugin for ChunksPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkMap>()
            .add_systems(Update, update_chunks.run_if(in_state(GameState::InGame)));
    }
}

fn player_pos_to_chunk_block(pos: Vec3) -> (IVec3, UVec3) {
    let size = Vec3::ONE * CHUNK_SIZE as f32;
    let pos = pos - size / 2.0;
    let chunk_pos = pos.div_euclid(size);
    let block_pos = pos.rem_euclid(size);
    (
        IVec3::new(chunk_pos.x as i32, chunk_pos.y as i32, chunk_pos.z as i32),
        UVec3::new(block_pos.x as u32, block_pos.y as u32, block_pos.z as u32),
    )
}

fn update_chunks(
    mut chunks_map: ResMut<ChunkMap>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    game_resources: Res<GameResources>,
    player_q: Query<&Transform, With<Player>>,
) {
    let Ok(transform) = player_q.get_single() else {
        return;
    };
    let (chunk_pos, _) = player_pos_to_chunk_block(transform.translation);
    let radius = DRAW_DISTANCE as i32;
    for x in -radius..radius {
        for z in -radius..radius {
            let chunk_pos = chunk_pos + x * IVec3::X + z * IVec3::Z;
            if !chunks_map.chunks.contains_key(&chunk_pos) {
                let chunk_pos_f32 =
                    Vec3::new(chunk_pos.x as f32, chunk_pos.y as f32, chunk_pos.z as f32);
                let chunk = generate_chunk(
                    chunk_pos_f32,
                    game_resources.blocks_map.clone(),
                    game_resources.blocks.clone(),
                );
                commands.spawn((
                    Mesh3d(meshes.add(chunk.clone())),
                    MeshMaterial3d(game_resources.material.clone()),
                    Transform::from_translation(chunk_pos_f32),
                ));
                chunks_map.chunks.insert(chunk_pos, chunk);
            }
        }
    }
}

#[derive(Debug, Resource)]
struct ChunkMap {
    chunks: HashMap<IVec3, Chunk>,
}

impl Default for ChunkMap {
    fn default() -> Self {
        ChunkMap {
            chunks: HashMap::new(),
        }
    }
}

impl ChunkMap {}

fn generate_chunk(
    pos: Vec3,
    block_map: Arc<HashMap<String, usize>>,
    blocks: Arc<Vec<Block>>,
) -> Chunk {
    let noise = noise::Perlin::new(SEED);
    let scale = 0.015;
    let mut chunk = Chunk::new(blocks);
    for x in 0..16 {
        let n_x = x as f64 + pos.x as f64;
        for z in 0..16 {
            let n_z = z as f64 + pos.z as f64;
            let n_y = (noise.get([n_x * scale, n_z * scale]) * 64.0).round() as i32;
            for y in 0..16 {
                let d = n_y - (y as i32 + pos.y.round() as i32);
                let block_id = match d {
                    0 => Some(block_map["grass"]),
                    0..3 => Some(block_map["dirt"]),
                    0.. => Some(block_map["stone"]),
                    _ => None,
                };
                chunk.set_at(UVec3::new(x, y, z), block_id);
            }
        }
    }
    chunk
}

fn index_to_pos(i: usize) -> UVec3 {
    let i = i as u32;
    let x = i / CHUNK_SIZE / CHUNK_SIZE;
    let i = i - x * CHUNK_SIZE / CHUNK_SIZE;
    let y = i / CHUNK_SIZE;
    let z = i - y * CHUNK_SIZE;
    UVec3::new(x, y, z)
}

#[derive(Debug, Clone)]
pub struct Chunk {
    blocks: [Option<usize>; CHUNK_LEN as usize],
    blocks_info: Arc<Vec<Block>>,
}

impl Chunk {
    fn new(blocks_info: Arc<Vec<Block>>) -> Self {
        Self {
            blocks: [None; CHUNK_LEN as usize],
            blocks_info,
        }
    }

    pub fn at(&self, pos: UVec3) -> Option<usize> {
        let i = pos.x * CHUNK_SIZE * CHUNK_SIZE + pos.y * CHUNK_SIZE + pos.z;
        self.blocks[i as usize]
    }

    pub fn set_at(&mut self, pos: UVec3, block: Option<usize>) {
        let i = pos.x * CHUNK_SIZE * CHUNK_SIZE + pos.y * CHUNK_SIZE + pos.z;
        self.blocks[i as usize] = block;
    }

    fn check_at(&self, pos: IVec3) -> bool {
        if pos.x < 0
            || pos.y < 0
            || pos.z < 0
            || pos.x >= CHUNK_SIZE as i32
            || pos.y >= CHUNK_SIZE as i32
            || pos.z >= CHUNK_SIZE as i32
        {
            return false;
        }
        self.at(UVec3::new(pos.x as u32, pos.y as u32, pos.z as u32))
            .is_some()
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.iter().flatten().count() == 0
    }
}

impl MeshBuilder for Chunk {
    fn build(&self) -> Mesh {
        let mut faces = self
            .blocks
            .iter()
            .enumerate()
            .filter_map(|(i, block_id)| block_id.map(|id| (i, &self.blocks_info[id])))
            .flat_map(|(i, block)| {
                let pos = index_to_pos(i);
                let pos = IVec3::new(pos.x as i32, pos.y as i32, pos.z as i32);
                let shift =
                    Vec3::new(pos.x as f32, pos.y as f32, pos.z as f32) + Vec3::ONE * CHUNK_OFFSET;
                [
                    (!self.check_at(pos + IVec3::Z))
                        .then_some(block.build_front_face_shifted(shift)),
                    (!self.check_at(pos - IVec3::Z))
                        .then_some(block.build_back_face_shifted(shift)),
                    (!self.check_at(pos + IVec3::X))
                        .then_some(block.build_right_face_shifted(shift)),
                    (!self.check_at(pos - IVec3::X))
                        .then_some(block.build_left_face_shifted(shift)),
                    (!self.check_at(pos + IVec3::Y)).then_some(block.build_top_face_shifted(shift)),
                    (!self.check_at(pos - IVec3::Y))
                        .then_some(block.build_bottom_face_shifted(shift)),
                ]
            })
            .flatten();
        let mesh = faces.next().unwrap();
        faces.fold(mesh, |mut mesh, face| {
            mesh.merge(&face);
            mesh
        })
    }
}

impl Meshable for Chunk {
    type Output = Chunk;

    fn mesh(&self) -> Self::Output {
        self.clone()
    }
}
