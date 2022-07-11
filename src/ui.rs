use bevy::prelude::*;

use crate::{
    components::{Health, HealthText, Player},
    enemies::enemy_movement,
};

// Constants
const TEXT_COLOR: Color = Color::WHITE;

// Plugin
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui)
            .add_system(update_health_ui.after(enemy_movement));
    }
}

// Systems
fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                margin: Rect::all(Val::Px(25.0)),
                ..default()
            },
            text: Text::with_section(
                "Health: %",
                TextStyle {
                    font,
                    font_size: 40.0,
                    color: TEXT_COLOR,
                },
                Default::default(),
            ),
            ..default()
        })
        .insert(HealthText);
}

fn update_health_ui(
    player_health: Query<&Health, With<Player>>,
    mut health_text: Query<&mut Text, With<HealthText>>,
) {
    let player_health = player_health.single();
    let mut health_text = health_text.single_mut();

    health_text.sections[0].value = format!("Health: {}", **player_health);
}
