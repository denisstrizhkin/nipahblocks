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
                let shift = Vec3::new(pos.x as f32, pos.y as f32, pos.z as f32) + Vec3::ONE * -7.5;
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
