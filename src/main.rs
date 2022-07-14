mod components;
mod enemies;
mod mouse;
mod player;
mod ui;
mod util;
mod weapons;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

use components::{InspectionPlugin, MainCamera};
use enemies::EnemyPlugin;
use mouse::MousePlugin;
use player::PlayerPlugin;
use ui::UiPlugin;
use weapons::WeaponPlugin;

// TODO:
// 1. Aiming + weapon damage
// 2. System refactoring
// 3. Reload UI
// 4. Weapon models (rectangles) NOTE: What about projectiles?
// 5. Score
// 6. Rolling
// 7. (Monster HP UI)
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
        .add_plugin(WeaponPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(MousePlugin)
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
