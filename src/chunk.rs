use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::PrimitiveTopology};

use crate::block::Block;

const CHUNK_SIZE: u32 = 16;

fn index_to_pos(i: usize) -> UVec3 {
    let i = i as u32;
    let x = i / CHUNK_SIZE / CHUNK_SIZE;
    let i = i - x * CHUNK_SIZE * CHUNK_SIZE;
    let y = i / CHUNK_SIZE;
    let z = i - y * CHUNK_SIZE;
    UVec3::new(x, y, z)
}

pub struct Chunk<'a> {
    blocks: [Option<&'a Block>; (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize],
}

impl Default for Chunk<'_> {
    fn default() -> Self {
        Self {
            blocks: [None; (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) as usize],
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
}

fn merge_mesh(mesh: Option<Mesh>, face_mesh: Mesh) -> Option<Mesh> {
    Some(match mesh {
        Some(mut mesh) => {
            mesh.merge(&face_mesh);
            mesh
        }
        None => face_mesh,
    })
}

impl MeshBuilder for Chunk<'_> {
    fn build(&self) -> Mesh {
        self.blocks
            .iter()
            .copied()
            .enumerate()
            .fold(Option::<Mesh>::None, |mut mesh, (i, block)| {
                if let Some(block) = block {
                    let pos = index_to_pos(i);
                    let pos = IVec3::new(pos.x as i32, pos.y as i32, pos.z as i32);
                    let shift = Vec3::new(pos.x as f32, pos.y as f32, pos.z as f32);
                    if !self.check_at(pos + IVec3::Z) {
                        let face_mesh = block.build_front_face_shifted(shift);
                        mesh = merge_mesh(mesh, face_mesh);
                    }
                    if !self.check_at(pos - IVec3::Z) {
                        let face_mesh = block.build_back_face_shifted(shift);
                        mesh = merge_mesh(mesh, face_mesh);
                    }
                    if !self.check_at(pos + IVec3::X) {
                        let face_mesh = block.build_right_face_shifted(shift);
                        mesh = merge_mesh(mesh, face_mesh);
                    }
                    if !self.check_at(pos - IVec3::X) {
                        let face_mesh = block.build_left_face_shifted(shift);
                        mesh = merge_mesh(mesh, face_mesh);
                    }
                    if !self.check_at(pos + IVec3::Y) {
                        let face_mesh = block.build_top_face_shifted(shift);
                        mesh = merge_mesh(mesh, face_mesh);
                    }
                    if !self.check_at(pos - IVec3::Y) {
                        let face_mesh = block.build_bottom_face_shifted(shift);
                        mesh = merge_mesh(mesh, face_mesh);
                    }
                }
                mesh
            })
            .unwrap()
    }
}

impl<'a> Meshable for Chunk<'a> {
    type Output = Chunk<'a>;

    fn mesh(&self) -> Self::Output {
        Self { ..*self }
    }
}
