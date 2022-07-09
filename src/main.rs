use std::f32::consts::PI;

use bevy::core::FixedTimestep;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use rand::random;

// #################
// ### Constants ###
// #################
const PLAYER_COLOR: Color = Color::BLUE;
const MONSTER_COLOR: Color = Color::RED;

const INITIAL_ENEMY_DISTANCE: f32 = 750.0;
const MAXIMUM_ENEMY_COUNT: usize = 10;

// ##################
// ### Components ###
// ##################
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Health(i32);

// TODO:
// 1. Monster movement
// 2. Player HP UI
// 3. Monster damage
// 4. (Monster HP UI)
// 5. First weapon (rectangles) NOTE: What about projectiles?
// 6. Weapon switching
// 7. Shooting + Aiming
// 8. Rolling
fn main() {
    App::new()
        // Resources
        .insert_resource(WindowDescriptor {
            title: "Cthulhu-Strike 1.6".to_string(),
            ..default()
        })
        // Startup Systems
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_player)
        // Systems
        .add_system(player_movement)
        .add_system(camera_lock.after(player_movement))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(enemy_spawner),
        )
        // Plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}

// ###############
// ### Systems ###
// ###############
fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

// Player Systems
fn spawn_player(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: PLAYER_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(30.0, 30.0, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Player)
        .insert(Health(5));
}

// TODO: Fix diagonal movement being faster than horizontal/vertical movement
fn player_movement(
    keys: Res<Input<KeyCode>>,
    mut player_transform: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = player_transform.single_mut();

    if keys.pressed(KeyCode::W) {
        player_transform.translation.y += 2.0;
    }
    if keys.pressed(KeyCode::A) {
        player_transform.translation.x -= 2.0;
    }
    if keys.pressed(KeyCode::S) {
        player_transform.translation.y -= 2.0;
    }
    if keys.pressed(KeyCode::D) {
        player_transform.translation.x += 2.0;
    }
}

fn camera_lock(
    player_transform: Query<&Transform, With<Player>>,
    mut camera_transform: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player_transform = player_transform.single();
    let mut camera_transform = camera_transform.single_mut();

    camera_transform.translation = player_transform.translation;
}

// Enemy Systems
fn spawn_enemy(mut commands: Commands, translation: Vec3) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: MONSTER_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(30.0, 30.0, 1.0),
                translation,
                ..default()
            },
            ..default()
        })
        .insert(Enemy)
        .insert(Health(25));
}

fn enemy_movement(
    player_transform: Query<&Transform, With<Player>>,
    enemy_transforms: Query<&mut Transform, With<Enemy>>,
) {
    let player_position = player_transform.single();

    for mut enemy_transform in enemy_transforms.iter() {
        // TODO: Move towards player
    }
}

fn enemy_spawner(
    commands: Commands,
    player_transform: Query<&Transform, With<Player>>,
    enemies: Query<Entity, With<Enemy>>,
) {
    let enemy_count = enemies.iter().count();

    // TODO: Consider removing this mechanic when it turns out to be less fun than endless spawning
    if enemy_count < MAXIMUM_ENEMY_COUNT {
        let player_transform = player_transform.single();

        // Create rotation Quad from rand
        let angle = random::<f32>() * 2.0 * PI;
        let x = INITIAL_ENEMY_DISTANCE * angle.cos();
        let y = INITIAL_ENEMY_DISTANCE * angle.sin();

        let enemy_translation = Vec3::new(
            player_transform.translation.x + x,
            player_transform.translation.y + y,
            0.0,
        );

        spawn_enemy(commands, enemy_translation);
    }
}
