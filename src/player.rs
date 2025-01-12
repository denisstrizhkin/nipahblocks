use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};
use std::f32::consts::FRAC_PI_2;

pub struct PlayerPlugin;

#[derive(Debug, Component)]
pub struct Player {
    movement_speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            movement_speed: 15.0,
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

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}

fn spawn_player(mut commands: Commands) {
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
    if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        direction += *transform.forward();
    }
    if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
        direction += *transform.back();
    }
    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        direction += *transform.left();
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        direction += *transform.right();
    }
    direction.y = 0.0;
    if keyboard.pressed(KeyCode::PageUp) || keyboard.pressed(KeyCode::Space) {
        direction += *transform.up();
    }
    if keyboard.pressed(KeyCode::PageDown) || keyboard.pressed(KeyCode::ControlLeft) {
        direction += *transform.down();
    }
    let movement = direction.normalize_or_zero() * player.movement_speed * time.delta_secs();
    transform.translation += movement;
}
