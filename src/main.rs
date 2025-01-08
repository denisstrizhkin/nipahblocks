use anyhow::{anyhow, Result};
use raylib::prelude::*;

const TILE_WIDTH: f32 = 16.0;

struct Block {
    front: u32,
    back: u32,
    left: u32,
    right: u32,
    top: u32,
    bottom: u32,
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
        }
    }

    pub fn generate_mesh(position: Vector3) -> Mesh {
        let mut mesh = allocate_mesh(2 * 6, 8);
        mesh
    }
}

fn allocate_mesh(triangle_count: usize, vertex_count: usize) -> Mesh {
    unsafe {
        let zeroed = std::mem::MaybeUninit::zeroed().assume_init();
        Mesh::from_raw(ffi::Mesh {
            vertexCount: vertex_count as i32,
            triangleCount: triangle_count as i32,
            vertices: ffi::MemAlloc((size_of::<[f32; 3]>() * vertex_count * 8) as u32).cast(),
            normals: ffi::MemAlloc((size_of::<[f32; 3]>() * vertex_count * 8) as u32).cast(),
            texcoords: ffi::MemAlloc((size_of::<[f32; 2]>() * vertex_count * 8) as u32).cast(),
            ..zeroed
        })
    }
}

// Generate a simple triangle mesh from code
fn gen_triangle_mesh() -> Mesh {
    let mut mesh = allocate_mesh(1, 3);

    // Vertex at (0, 0, 0)
    mesh.vertices_mut()[0].x = 0.0;
    mesh.vertices_mut()[0].y = 0.0;
    mesh.vertices_mut()[0].z = 0.0;
    mesh.normals_mut()[0].x = 0.0;
    mesh.normals_mut()[0].y = 1.0;
    mesh.normals_mut()[0].z = 0.0;
    unsafe {
        *mesh.texcoords.add(0) = 0.0;
        *mesh.texcoords.add(1) = 0.0;
    }

    // Vertex at (1, 0, 0)
    mesh.vertices_mut()[1].x = 1.0;
    mesh.vertices_mut()[1].y = 0.0;
    mesh.vertices_mut()[1].z = 0.0;
    mesh.normals_mut()[1].x = 0.0;
    mesh.normals_mut()[1].y = 1.0;
    mesh.normals_mut()[1].z = 0.0;
    unsafe {
        *mesh.texcoords.add(2) = TILE_WIDTH / 160.0;
        *mesh.texcoords.add(3) = 0.0;
    }

    // // Vertex at (0, 1, 0)
    mesh.vertices_mut()[2].x = 0.0;
    mesh.vertices_mut()[2].y = 1.0;
    mesh.vertices_mut()[2].z = 0.0;
    mesh.normals_mut()[2].x = 0.0;
    mesh.normals_mut()[2].y = 1.0;
    mesh.normals_mut()[2].z = 0.0;
    unsafe {
        *mesh.texcoords.add(4) = 0.0;
        *mesh.texcoords.add(5) = TILE_WIDTH / 256.0;
    }

    // Upload mesh data from CPU (RAM) to GPU (VRAM) memory
    unsafe {
        mesh.upload(false);
    }
    mesh
}

fn main() -> Result<()> {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    let tilemap = rl
        .load_texture(&thread, "assets/tilemap.png")
        .map_err(|e| anyhow!(e))?;
    let rect = Rectangle::new(TILE_WIDTH * 3.0, 0.0, TILE_WIDTH, TILE_WIDTH);

    let mesh = gen_triangle_mesh();
    let model = unsafe {
        let mut model = rl
            .load_model_from_mesh(&thread, mesh.make_weak())
            .map_err(|e| anyhow!(e))?;
        model.materials_mut()[0].maps_mut()[MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize]
            .texture = *tilemap.as_ref();
        model
    };
    let model_pos = rvec3(0.0, 0.0, 0.0);

    let mut camera = Camera3D::perspective(
        rvec3(5.0, 5.0, 5.0),
        rvec3(0.0, 0.0, 0.0),
        rvec3(0.0, 1.0, 0.0),
        45.0,
    );
    rl.set_target_fps(60);
    while !rl.window_should_close() {
        rl.update_camera(&mut camera, CameraMode::CAMERA_FREE);
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        {
            let mut d = d.begin_mode3D(camera);

            // d.draw_cube_v(rvec3(0.0, 0.0, 0.0), rvec3(2.0, 2.0, 2.0), Color::RED);
            d.draw_grid(10, 1.0);
            d.draw_model(&model, model_pos, 1.0, Color::WHITE);
        }

        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
        d.draw_texture_rec(&tilemap, rect, rvec2(0.0, 0.0), Color::WHITE);
    }
    Ok(())
}
