use bevy::{
    asset::RenderAssetUsages,
    math::Vec2,
    prelude::{Mesh, MeshBuilder, Meshable},
    render::mesh::{Indices, PrimitiveTopology},
};

const TEXTURE_SIZE: u32 = 16;

pub struct Block {
    front: u32,
    back: u32,
    left: u32,
    right: u32,
    top: u32,
    bottom: u32,
    width: f32,
}

impl Block {
    pub fn new(front: u32, back: u32, left: u32, right: u32, top: u32, bottom: u32) -> Self {
        Self {
            front,
            back,
            left,
            right,
            top,
            bottom,
            width: 0.5,
        }
    }

    fn get_uvs(face: u32) -> (Vec2, Vec2) {
        let pos = Vec2::X * face as f32;
        let size = TEXTURE_SIZE as f32;
        let min = pos / size;
        let max = (pos + Vec2::ONE) / size;
        (min, max)
    }

    fn build_front_face(&self) -> [([f32; 3], [f32; 3], [f32; 2]); 4] {
        let normal = [0.0, 0.0, 1.0];
        let (min, max) = (-self.width, self.width);
        let (uv_min, uv_max) = Block::get_uvs(self.front);
        [
            ([min, min, max], normal, [uv_min.x, uv_max.y]),
            ([max, min, max], normal, [uv_max.x, uv_max.y]),
            ([max, max, max], normal, [uv_max.x, uv_min.y]),
            ([min, max, max], normal, [uv_min.x, uv_min.y]),
        ]
    }
}

impl MeshBuilder for Block {
    fn build(&self) -> Mesh {
        let min = -0.5;
        let max = 0.5;

        // Suppose Y-up right hand, and camera look from +Z to -Z
        let vertices_other = &[
            // Back
            ([min, max, min], [0.0, 0.0, -1.0], [1.0, 0.0]),
            ([max, max, min], [0.0, 0.0, -1.0], [0.0, 0.0]),
            ([max, min, min], [0.0, 0.0, -1.0], [0.0, 1.0]),
            ([min, min, min], [0.0, 0.0, -1.0], [1.0, 1.0]),
            // Right
            ([max, min, min], [1.0, 0.0, 0.0], [0.0, 0.0]),
            ([max, max, min], [1.0, 0.0, 0.0], [1.0, 0.0]),
            ([max, max, max], [1.0, 0.0, 0.0], [1.0, 1.0]),
            ([max, min, max], [1.0, 0.0, 0.0], [0.0, 1.0]),
            // Left
            ([min, min, max], [-1.0, 0.0, 0.0], [1.0, 0.0]),
            ([min, max, max], [-1.0, 0.0, 0.0], [0.0, 0.0]),
            ([min, max, min], [-1.0, 0.0, 0.0], [0.0, 1.0]),
            ([min, min, min], [-1.0, 0.0, 0.0], [1.0, 1.0]),
            // Top
            ([max, max, min], [0.0, 1.0, 0.0], [1.0, 0.0]),
            ([min, max, min], [0.0, 1.0, 0.0], [0.0, 0.0]),
            ([min, max, max], [0.0, 1.0, 0.0], [0.0, 1.0]),
            ([max, max, max], [0.0, 1.0, 0.0], [1.0, 1.0]),
            // Bottom
            ([max, min, max], [0.0, -1.0, 0.0], [0.0, 0.0]),
            ([min, min, max], [0.0, -1.0, 0.0], [1.0, 0.0]),
            ([min, min, min], [0.0, -1.0, 0.0], [1.0, 1.0]),
            ([max, min, min], [0.0, -1.0, 0.0], [0.0, 1.0]),
        ];
        let vertices = self.build_front_face();

        let positions: Vec<_> = vertices
            .iter()
            .chain(vertices_other)
            .map(|(p, _, _)| *p)
            .collect();
        let normals: Vec<_> = vertices
            .iter()
            .chain(vertices_other)
            .map(|(_, n, _)| *n)
            .collect();
        let uvs: Vec<_> = vertices
            .iter()
            .chain(vertices_other)
            .map(|(_, _, uv)| *uv)
            .collect();

        let indices = Indices::U32(vec![
            0, 1, 2, 2, 3, 0, // front
            4, 5, 6, 6, 7, 4, // back
            8, 9, 10, 10, 11, 8, // right
            12, 13, 14, 14, 15, 12, // left
            16, 17, 18, 18, 19, 16, // top
            20, 21, 22, 22, 23, 20, // bottom
        ]);

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(indices)
    }
}

impl Meshable for Block {
    type Output = Block;

    fn mesh(&self) -> Self::Output {
        Block { ..*self }
    }
}
