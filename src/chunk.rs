use bevy::prelude::*;

use crate::block::Block;

const CHUNK_SIZE: usize = 16;

pub struct Chunk<'a> {
    blocks: [Option<&'a Block>; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
}

impl<'a> Chunk<'a> {
    pub fn at(&self, pos: UVec3) -> Option<&'a Block> {
        self.blocks[pos.x as usize * CHUNK_SIZE * CHUNK_SIZE
            + pos.y as usize * CHUNK_SIZE
            + pos.z as usize]
    }

    pub fn set_at(&mut self, pos: UVec3, block: Option<&'a Block>) {
        self.blocks[pos.x as usize * CHUNK_SIZE * CHUNK_SIZE
            + pos.y as usize * CHUNK_SIZE
            + pos.z as usize] = block;
    }
}

impl<'a> MeshBuilder for Chunk<'a> {
    fn build(&self) -> Mesh {
        todo!()
    }
}

impl<'a> Meshable for Chunk<'a> {
    type Output = Chunk<'a>;

    fn mesh(&self) -> Self::Output {
        Self { ..*self }
    }
}
