use bevy::{prelude::*, utils::hashbrown::HashMap};
use noise::NoiseFn;

use crate::block::{Block, BLOCK_HALF_SIZE};
use crate::player::Player;
use crate::GameResources;

const CHUNK_SIZE: u32 = 16;
const CHUNK_LEN: u32 = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;
const CHUNK_OFFSET: f32 = -(CHUNK_SIZE as f32 / 2.0) + BLOCK_HALF_SIZE;

const SEED: u32 = 123456;

const DRAW_DISTANCE: u32 = 2;

pub struct ChunksPlugin;

impl Plugin for ChunksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_chunks_map)
            .add_systems(Update, update_chunks);
    }
}

fn setup_chunks_map(game_resources: Res<GameResources>, mut commands: Commands) {
    commands.insert_resource(ChunkMap::new(&game_resources.blocks));
}

fn update_chunks(
    mut chunks_map: ResMut<ChunkMap>,
    mut commands: Commands,
    player_q: Query<&Transform, With<Player>>,
) {
    let Ok(transform) = player_q.get_single() else {
        return;
    };
}

#[derive(Debug, Resource)]
struct ChunkMap<'a> {
    chunks: HashMap<IVec3, Chunk<'a>>,
    blocks: HashMap<String, Block>,
}

impl<'a> ChunkMap<'a> {
    fn new(blocks: &HashMap<String, Block>) -> Self {
        ChunkMap {
            chunks: HashMap::new(),
            blocks: blocks.clone(),
        }
    }

    pub fn chunk_at(&'a mut self, pos: Vec3) -> &'a Chunk {
        let size = Vec3::ONE * CHUNK_SIZE as f32;
        let pos = pos - size / 2.0;
        let pos = pos.div_euclid(size);
        let ipos = IVec3::new(pos.x as i32, pos.y as i32, pos.z as i32);
        if !self.chunks.contains_key(&ipos) {
            let chunk = self.generate_chunk(pos);
            self.chunks.insert(ipos, chunk);
        }
        &self.chunks[&ipos]
    }

    pub fn block_at(&'a mut self, pos: Vec3) -> Option<&'a Block> {
        let chunk = self.chunk_at(pos);
        let size = Vec3::ONE * CHUNK_SIZE as f32;
        let pos = pos - size / 2.0;
        let pos = pos.rem_euclid(size);
        let upos = UVec3::new(pos.x as u32, pos.y as u32, pos.z as u32);
        chunk.at(upos)
    }

    fn generate_chunk(&self, pos: Vec3) -> Chunk<'a> {
        let noise = noise::Perlin::new(SEED);
        let scale = 0.015;
        let mut chunk = Chunk::default();
        for x in 0..16 {
            let n_x = x as f64 + pos.x as f64;
            for z in 0..16 {
                let n_z = z as f64 + pos.z as f64;
                let n_y = (noise.get([n_x * scale, n_z * scale]) * 64.0).round() as i32;
                for y in 0..16 {
                    let d = n_y - (y as i32 + pos.y.round() as i32);
                    let block = match d {
                        0 => Some(&self.blocks["grass"]),
                        0..3 => Some(&self.blocks["dirt"]),
                        0.. => Some(&self.blocks["stone"]),
                        _ => None,
                    };
                    chunk.set_at(UVec3::new(x, y, z), block);
                }
            }
        }
        chunk
    }
}

fn index_to_pos(i: usize) -> UVec3 {
    let i = i as u32;
    let x = i / CHUNK_SIZE / CHUNK_SIZE;
    let i = i - x * CHUNK_SIZE / CHUNK_SIZE;
    let y = i / CHUNK_SIZE;
    let z = i - y * CHUNK_SIZE;
    UVec3::new(x, y, z)
}

#[derive(Debug)]
pub struct Chunk<'a> {
    blocks: [Option<&'a Block>; CHUNK_LEN as usize],
}

impl Default for Chunk<'_> {
    fn default() -> Self {
        Self {
            blocks: [None; CHUNK_LEN as usize],
        }
    }
}

impl<'a> Chunk<'a> {
    pub fn at(&self, pos: UVec3) -> Option<&'a Block> {
        let i = pos.x * CHUNK_SIZE * CHUNK_SIZE + pos.y * CHUNK_SIZE + pos.z;
        self.blocks[i as usize]
    }

    pub fn set_at(&mut self, pos: UVec3, block: Option<&'a Block>) {
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

impl MeshBuilder for Chunk<'_> {
    fn build(&self) -> Mesh {
        let mut faces = self
            .blocks
            .iter()
            .enumerate()
            .filter_map(|(i, block)| block.map(|block| (i, block)))
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

impl<'a> Meshable for Chunk<'a> {
    type Output = Chunk<'a>;

    fn mesh(&self) -> Self::Output {
        Self { ..*self }
    }
}
