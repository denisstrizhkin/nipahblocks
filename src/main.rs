use anyhow::{anyhow, Result};
use bevy::{
    input::mouse::AccumulatedMouseMotion,
    pbr::wireframe::{Wireframe, WireframeColor},
    prelude::*,
};

use std::f32::consts::FRAC_PI_2;

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

    // pub fn generate_mesh(position: Vector3) -> Mesh {
    //     let mut mesh = allocate_mesh(2 * 6, 2 * 6 * 3);

    //     // front
    //     mesh.vertices_mut()[0].x = 0.0;
    //     mesh.vertices_mut()[0].y = 0.0;
    //     mesh.vertices_mut()[0].z = 0.0;
    //     mesh.normals_mut()[0].x = 0.0;
    //     mesh.normals_mut()[0].y = 0.0;
    //     mesh.normals_mut()[0].z = 1.0;
    //     unsafe {
    //         *mesh.texcoords.add(0) = 0.0;
    //         *mesh.texcoords.add(1) = 0.0;
    //     }

    //     mesh.vertices_mut()[1].x = 1.0;
    //     mesh.vertices_mut()[1].y = 0.0;
    //     mesh.vertices_mut()[1].z = 0.0;
    //     mesh.normals_mut()[1].x = 0.0;
    //     mesh.normals_mut()[1].y = 0.0;
    //     mesh.normals_mut()[1].z = 1.0;
    //     unsafe {
    //         *mesh.texcoords.add(2) = TILE_WIDTH / 160.0;
    //         *mesh.texcoords.add(3) = 0.0;
    //     }

    //     mesh.vertices_mut()[2].x = 0.0;
    //     mesh.vertices_mut()[2].y = 1.0;
    //     mesh.vertices_mut()[2].z = 0.0;
    //     mesh.normals_mut()[2].x = 0.0;
    //     mesh.normals_mut()[2].y = 0.0;
    //     mesh.normals_mut()[2].z = 1.0;
    //     unsafe {
    //         *mesh.texcoords.add(4) = 0.0;
    //         *mesh.texcoords.add(5) = TILE_WIDTH / 256.0;
    //     }

    //     unsafe {
    //         mesh.upload(false);
    //     }
    //     mesh
    // }
}

#[derive(Debug, Component)]
struct Player {
    movement_speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            movement_speed: 5.0,
        }
    }
}

#[derive(Debug, Component, Deref, DerefMut)]
struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.002, 0.002))
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_view_model, setup))
        .add_systems(Update, move_player)
        .run();
}

fn spawn_view_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Player::default(),
            CameraSensitivity::default(),
            Transform::from_xyz(2.0, 0.5, 2.0),
            Visibility::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 45.0_f32.to_radians(),
                    ..default()
                }),
            ));
        });
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::from_length(1.0))),
        MeshMaterial3d(materials.add(Color::BLACK)),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Wireframe,
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

fn move_player(
    time: Res<Time>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<(&mut Transform, &Player, &CameraSensitivity), With<Player>>,
) {
    let Ok((mut transform, player, camera_sensitivity)) = player_q.get_single_mut() else {
        return;
    };
    let delta = accumulated_mouse_motion.delta;
    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
    let mut direction = Vec3::ZERO;
    if keyboard.pressed(KeyCode::ArrowUp) {
        direction += *transform.forward();
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        direction += *transform.back();
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        direction += *transform.left();
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        direction += *transform.right();
    }
    direction.y = 0.0;
    let movement = direction.normalize_or_zero() * player.movement_speed * time.delta_secs();
    transform.translation += movement;
}
