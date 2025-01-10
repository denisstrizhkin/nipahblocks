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

impl<'a> Chunk<'a> {
    pub fn at(&self, pos: UVec3) -> Option<&'a Block> {
        match pos.to_array() {
            [x @ 0..CHUNK_SIZE, y @ 0..CHUNK_SIZE, z @ 0..CHUNK_SIZE] => {
                let i = x * CHUNK_SIZE * CHUNK_SIZE + y * CHUNK_SIZE + z;
                self.blocks[i as usize]
            }
            _ => None,
        }
    }

    pub fn set_at(&mut self, pos: UVec3, block: Option<&'a Block>) {
        let i = pos.x * CHUNK_SIZE * CHUNK_SIZE + pos.y * CHUNK_SIZE + pos.z;
        self.blocks[i as usize] = block;
    }
}

impl MeshBuilder for Chunk<'_> {
    fn build(&self) -> Mesh {
        self.blocks.iter().copied().enumerate().fold(
            Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::default(),
            ),
            |mut mesh, (i, block)| {
                if let Some(block) = block {
                    let pos = index_to_pos(i);
                    let shift = Vec3::new(pos.x as f32, pos.y as f32, pos.z as f32);
                    if self.at(pos + UVec3::Z).is_none() {
                        mesh.merge(&block.build_front_face_shifted(shift))
                    }
                    if self.at(pos - UVec3::Z).is_none() {
                        mesh.merge(&block.build_back_face_shifted(shift))
                    }
                    if self.at(pos + UVec3::X).is_none() {
                        mesh.merge(&block.build_left_face_shifted(shift))
                    }
                    if self.at(pos - UVec3::X).is_none() {
                        mesh.merge(&block.build_right_face_shifted(shift))
                    }
                    if self.at(pos + UVec3::Y).is_none() {
                        mesh.merge(&block.build_top_face_shifted(shift))
                    }
                    if self.at(pos - UVec3::Y).is_none() {
                        mesh.merge(&block.build_bottom_face_shifted(shift))
                    }
                }
                mesh
            },
        )
    }
}

impl<'a> Meshable for Chunk<'a> {
    type Output = Chunk<'a>;

    fn mesh(&self) -> Self::Output {
        Self { ..*self }
    }
}
