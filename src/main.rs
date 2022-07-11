mod components;
mod enemies;
mod player;
mod ui;
mod util;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

use components::{InspectionPlugin, MainCamera};
use enemies::EnemyPlugin;
use player::PlayerPlugin;
use ui::UiPlugin;

// #################
// ### Constants ###
// #################
// TODO: Stats (Weapon)

// #################
// ### Resources ###
// #################
#[derive(Default)]
struct PlayerInvincibility(f32);

// TODO:
// 1. Monster damage
// 2. (Monster HP UI)
// 3. Weapon switching
// 4. First weapon (rectangles) NOTE: What about projectiles?
// 5. Shooting + Aiming
// 6. Rolling
// 7. Score
fn main() {
    App::new()
        // Resources
        .insert_resource(WindowDescriptor {
            title: "Cthulhu-Strike 1.6".to_string(),
            width: 1280.0,
            height: 720.0,
            ..default()
        })
        // Startup Systems
        .add_startup_system(setup_camera)
        // Plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InspectionPlugin)
        .run();
}

// Systems
fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera)
        .insert(Name::new("MainCamera"));
}
