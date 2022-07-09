use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

// Constants
const PLAYER_COLOR: Color = Color::BLUE;
const MONSTER_COLOR: Color = Color::RED;

// Components
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Health(i32);

// TODO:
// 1. Monster spawner (red tiles)
// 2. Monster movement
// 3. Monster damage
// 4. First weapon (rectangles) NOTE: What about projectiles?
// 5. Weapon switching
// 6. Shooting + Aiming
// 7. Rolling
fn main() {
    App::new()
        // Startup Systems
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemy)
        // Systems
        .add_system(player_movement)
        // Plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}

// Systems
fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: PLAYER_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(Player)
        .insert(Health(5));
}

fn spawn_enemy(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: MONSTER_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                translation: Vec3::new(20.0, 20.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Enemy)
        .insert(Health(25));
}

// TODO: Fix diagonal movement being faster than horizontal/vertical movement
fn player_movement(keys: Res<Input<KeyCode>>, mut q: Query<&mut Transform, With<Player>>) {
    let mut player_transform = q.single_mut();

    if keys.pressed(KeyCode::W) {
        player_transform.translation.y += 2.;
    }
    if keys.pressed(KeyCode::A) {
        player_transform.translation.x -= 2.;
    }
    if keys.pressed(KeyCode::S) {
        player_transform.translation.y -= 2.;
    }
    if keys.pressed(KeyCode::D) {
        player_transform.translation.x += 2.;
    }
}
