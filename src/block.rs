use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

const TEXTURE_SIZE: u32 = 16;

#[derive(Debug, Clone)]
pub struct Block {
    front: Rect,
    back: Rect,
    left: Rect,
    right: Rect,
    top: Rect,
    bottom: Rect,
    width: f32,
}

impl Block {
    pub fn new(front: Rect, back: Rect, left: Rect, right: Rect, top: Rect, bottom: Rect) -> Self {
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

    fn build_front_face(&self) -> [([f32; 3], [f32; 3], [f32; 2]); 4] {
        let normal = [0.0, 0.0, 1.0];
        let (min, max) = (-self.width, self.width);
        let (uv_min, uv_max) = (self.front.min, self.front.max);
        [
            ([min, min, max], normal, [uv_min.x, uv_max.y]),
            ([max, min, max], normal, [uv_max.x, uv_max.y]),
            ([max, max, max], normal, [uv_max.x, uv_min.y]),
            ([min, max, max], normal, [uv_min.x, uv_min.y]),
        ]
    }

    fn build_back_face(&self) -> [([f32; 3], [f32; 3], [f32; 2]); 4] {
        let normal = [0.0, 0.0, -1.0];
        let (min, max) = (-self.width, self.width);
        let (uv_min, uv_max) = (self.back.min, self.back.max);
        [
            ([min, max, min], normal, [uv_max.x, uv_min.y]),
            ([max, max, min], normal, [uv_min.x, uv_min.y]),
            ([max, min, min], normal, [uv_min.x, uv_max.y]),
            ([min, min, min], normal, [uv_max.x, uv_max.y]),
        ]
    }
    fn build_right_face(&self) -> [([f32; 3], [f32; 3], [f32; 2]); 4] {
        let normal = [1.0, 0.0, 0.0];
        let (min, max) = (-self.width, self.width);
        let (uv_min, uv_max) = (self.right.min, self.right.max);
        [
            ([max, min, min], normal, [uv_max.x, uv_max.y]),
            ([max, max, min], normal, [uv_max.x, uv_min.y]),
            ([max, max, max], normal, [uv_min.x, uv_min.y]),
            ([max, min, max], normal, [uv_min.x, uv_max.y]),
        ]
    }
    fn build_left_face(&self) -> [([f32; 3], [f32; 3], [f32; 2]); 4] {
        let normal = [-1.0, 0.0, 0.0];
        let (min, max) = (-self.width, self.width);
        let (uv_min, uv_max) = (self.left.min, self.left.max);
        [
            ([min, min, max], normal, [uv_max.x, uv_max.y]),
            ([min, max, max], normal, [uv_max.x, uv_min.y]),
            ([min, max, min], normal, [uv_min.x, uv_min.y]),
            ([min, min, min], normal, [uv_min.x, uv_max.y]),
        ]
    }
    fn build_top_face(&self) -> [([f32; 3], [f32; 3], [f32; 2]); 4] {
        let normal = [0.0, 1.0, 0.0];
        let (min, max) = (-self.width, self.width);
        let (uv_min, uv_max) = (self.top.min, self.top.max);
        [
            ([max, max, min], normal, [uv_max.x, uv_min.y]),
            ([min, max, min], normal, [uv_min.x, uv_min.y]),
            ([min, max, max], normal, [uv_min.x, uv_max.y]),
            ([max, max, max], normal, [uv_max.x, uv_max.y]),
        ]
    }
    fn build_bottom_face(&self) -> [([f32; 3], [f32; 3], [f32; 2]); 4] {
        let normal = [0.0, -1.0, 0.0];
        let (min, max) = (-self.width, self.width);
        let (uv_min, uv_max) = (self.bottom.min, self.bottom.max);
        [
            ([max, min, max], normal, [uv_max.x, uv_max.y]),
            ([min, min, max], normal, [uv_max.x, uv_min.y]),
            ([min, min, min], normal, [uv_min.x, uv_min.y]),
            ([max, min, min], normal, [uv_min.x, uv_max.y]),
        ]
    }
}

impl MeshBuilder for Block {
    fn build(&self) -> Mesh {
        let vertices = self
            .build_front_face()
            .into_iter()
            .chain(self.build_back_face())
            .chain(self.build_right_face())
            .chain(self.build_left_face())
            .chain(self.build_top_face())
            .chain(self.build_bottom_face())
            .collect::<Vec<_>>();

        let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
        let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
        let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

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
