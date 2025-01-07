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

fn main() -> Result<()> {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();
    let tilemap = rl
        .load_texture(&thread, "assets/tilemap.png")
        .map_err(|e| anyhow!(e))?;
    let width = 16.0;
    let rect = Rectangle::new(width * 3.0, 0.0, width, width);
    println!("{tilemap:?}");

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
        d.draw_texture_rec(&tilemap, rect, Vector2::new(0.0, 0.0), Color::WHITE);
    }
    Ok(())
}
