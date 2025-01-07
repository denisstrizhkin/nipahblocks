use anyhow::{anyhow, Result};
use raylib::prelude::*;

struct Block {
    front: u32,
    back: u32,
    left: u32,
    right: u32,
    up: u32,
    down: u32,
}

fn allocate_mesh(triangle_count: usize, vertex_count: usize) -> Mesh {
    unsafe {
        let zeroed = std::mem::MaybeUninit::zeroed().assume_init();
        Mesh::from_raw(ffi::Mesh {
            vertexCount: triangle_count as i32,
            triangleCount: vertex_count as i32,
            vertices: ffi::MemAlloc((size_of::<[f32; 3]>() * vertex_count) as u32).cast(),
            texcoords: ffi::MemAlloc((size_of::<[f32; 2]>() * vertex_count) as u32).cast(),
            normals: ffi::MemAlloc((size_of::<[f32; 3]>() * vertex_count) as u32).cast(),
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
    // mesh.texcoords[0] = 0;
    // mesh.texcoords[1] = 0;

    // Vertex at (1, 0, 2)
    mesh.vertices_mut()[0].x = 1.0;
    mesh.vertices_mut()[0].y = 0.0;
    mesh.vertices_mut()[0].z = 2.0;
    mesh.normals_mut()[0].x = 0.0;
    mesh.normals_mut()[0].y = 1.0;
    mesh.normals_mut()[0].z = 0.0;
    // mesh.texcoords[2] = 0.5f;
    // mesh.texcoords[3] = 1.0f;

    // // Vertex at (2, 0, 0)
    mesh.vertices_mut()[0].x = 2.0;
    mesh.vertices_mut()[0].y = 0.0;
    mesh.vertices_mut()[0].z = 0.0;
    mesh.normals_mut()[0].x = 0.0;
    mesh.normals_mut()[0].y = 1.0;
    mesh.normals_mut()[0].z = 0.0;
    // mesh.texcoords[4] = 1;
    // mesh.texcoords[5] = 0;

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
    let width = 16.0;
    let rect = Rectangle::new(width * 3.0, 0.0, width, width);
    println!("{tilemap:?}");

    let mesh = gen_triangle_mesh();
    let model = unsafe {
        rl.load_model_from_mesh(&thread, mesh.make_weak())
            .map_err(|e| anyhow!(e))?
    };
    let model_pos = Vector3::new(0.0, 0.0, 0.0);

    let camera = Camera3D::perspective(
        rvec3(5.0, 5.0, 5.0),
        rvec3(0.0, 0.0, 0.0),
        rvec3(0.0, 1.0, 0.0),
        45.0,
    );
    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        {
            let mut d = d.begin_mode3D(&camera);

            d.draw_model(&model, model_pos, 1.0, Color::BLACK);

            d.draw_grid(10, 1.0);
        }

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
        d.draw_texture_rec(&tilemap, rect, Vector2::new(0.0, 0.0), Color::WHITE);
    }
    Ok(())
}
