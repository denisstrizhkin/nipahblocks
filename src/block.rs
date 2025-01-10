use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

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

fn build_face_mesh(verices: Vec<[f32; 3]>, normal: [f32; 3], uvs: Vec<[f32; 2]>) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, verices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![normal, normal, normal, normal])
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(vec![0, 1, 2, 2, 3, 0]))
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

    fn build_front_face(&self) -> Mesh {
        let normal = [0.0, 0.0, 1.0];
        let (min, max) = (-self.width, self.width);
        let (uv_min, uv_max) = (self.front.min, self.front.max);
        build_face_mesh(
            vec![
                [min, min, max],
                [max, min, max],
                [max, max, max],
                [min, max, max],
            ],
            normal,
            vec![
                [uv_min.x, uv_max.y],
                [uv_max.x, uv_max.y],
                [uv_max.x, uv_min.y],
                [uv_min.x, uv_min.y],
            ],
        )
    }

    fn build_back_face(&self) -> Mesh {
        let normal = [0.0, 0.0, -1.0];
        let (min, max) = (-self.width, self.width);
        let (uv_min, uv_max) = (self.back.min, self.back.max);
        build_face_mesh(
            vec![
                [min, max, min],
                [max, max, min],
                [max, min, min],
                [min, min, min],
            ],
            normal,
            vec![
                [uv_max.x, uv_min.y],
                [uv_min.x, uv_min.y],
                [uv_min.x, uv_max.y],
                [uv_max.x, uv_max.y],
            ],
        )
    }

    fn build_right_face(&self) -> Mesh {
        let normal = [1.0, 0.0, 0.0];
        let (min, max) = (-self.width, self.width);
        let (uv_min, uv_max) = (self.right.min, self.right.max);
        build_face_mesh(
            vec![
                [max, min, min],
                [max, max, min],
                [max, max, max],
                [max, min, max],
            ],
            normal,
            vec![
                [uv_max.x, uv_max.y],
                [uv_max.x, uv_min.y],
                [uv_min.x, uv_min.y],
                [uv_min.x, uv_max.y],
            ],
        )
    }
    fn build_left_face(&self) -> Mesh {
        let normal = [-1.0, 0.0, 0.0];
        let (min, max) = (-self.width, self.width);
        let (uv_min, uv_max) = (self.left.min, self.left.max);
        build_face_mesh(
            vec![
                [min, min, max],
                [min, max, max],
                [min, max, min],
                [min, min, min],
            ],
            normal,
            vec![
                [uv_max.x, uv_max.y],
                [uv_max.x, uv_min.y],
                [uv_min.x, uv_min.y],
                [uv_min.x, uv_max.y],
            ],
        )
    }
    fn build_top_face(&self) -> Mesh {
        let normal = [0.0, 1.0, 0.0];
        let (min, max) = (-self.width, self.width);
        let (uv_min, uv_max) = (self.top.min, self.top.max);
        build_face_mesh(
            vec![
                [max, max, min],
                [min, max, min],
                [min, max, max],
                [max, max, max],
            ],
            normal,
            vec![
                [uv_max.x, uv_min.y],
                [uv_min.x, uv_min.y],
                [uv_min.x, uv_max.y],
                [uv_max.x, uv_max.y],
            ],
        )
    }
    fn build_bottom_face(&self) -> Mesh {
        let normal = [0.0, -1.0, 0.0];
        let (min, max) = (-self.width, self.width);
        let (uv_min, uv_max) = (self.bottom.min, self.bottom.max);
        build_face_mesh(
            vec![
                [max, min, max],
                [min, min, max],
                [min, min, min],
                [max, min, min],
            ],
            normal,
            vec![
                [uv_max.x, uv_max.y],
                [uv_max.x, uv_min.y],
                [uv_min.x, uv_min.y],
                [uv_min.x, uv_max.y],
            ],
        )
    }
}

impl MeshBuilder for Block {
    fn build(&self) -> Mesh {
        let mut mesh = self.build_front_face();
        mesh.merge(&self.build_back_face());
        mesh.merge(&self.build_right_face());
        mesh.merge(&self.build_left_face());
        mesh.merge(&self.build_top_face());
        mesh.merge(&self.build_bottom_face());
        mesh
    }
}

impl Meshable for Block {
    type Output = Block;

    fn mesh(&self) -> Self::Output {
        Block { ..*self }
    }
}
