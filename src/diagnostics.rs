use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::player::Player;

#[derive(Component)]
struct DiagnosticsText;

pub struct DiagnosticsPlugin;

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Startup, setup_diagnostics)
            .add_systems(Update, update_diagnostics);
    }
}

fn setup_diagnostics(mut commands: Commands) {
    // Text with multiple sections
    commands.spawn(Node::default()).with_children(|builder| {
        builder.spawn((
            DiagnosticsText,
            Text::new("Diagnostics:"),
            TextColor(Color::WHITE),
            TextFont {
                font_size: 30.0,
                ..default()
            },
        ));
    });
}
fn update_diagnostics(
    diagnostics: Res<DiagnosticsStore>,
    player_q: Query<&Transform, With<Player>>,
    mut text_q: Query<&mut Text, With<DiagnosticsText>>,
) {
    let mut s = String::new();
    if let Ok(transform) = player_q.get_single() {
        s += format!("Position - {}, ", transform.translation).as_str();
    };
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            s += format!("FPS: - {value:.3}, ").as_str();
        }
    }
    if let Ok(mut text) = text_q.get_single_mut() {
        **text = s;
    }
}
