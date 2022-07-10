use std::f32::consts::PI;

use bevy::core::FixedTimestep;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};
use rand::random;

// #################
// ### Constants ###
// #################
// Colors
const PLAYER_COLOR: Color = Color::BLUE;
const ENEMY_COLOR: Color = Color::RED;
const TEXT_COLOR: Color = Color::WHITE;

// Stats (Player)
const PLAYER_SPEED: f32 = 2.0;
const PLAYER_HEALTH: i32 = 5;
// Stats (Enemy)
const ENEMY_SPEED: f32 = 1.5;
const ENEMY_HEALTH: i32 = 25;

// TODO: Stats (Weapon)

// Miscellaneous
const INITIAL_ENEMY_DISTANCE: f32 = 750.0;
const MAXIMUM_ENEMY_COUNT: usize = 10;

// ##################
// ### Components ###
// ##################
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component, Deref, DerefMut, Inspectable)]
struct Health(i32);

#[derive(Component)]
struct HealthText;

#[derive(Component, Deref, DerefMut, Inspectable)]
struct Speed(f32);

#[derive(Component)]
struct MainCamera;

// #################
// ### Resources ###
// #################
#[derive(Default)]
struct PlayerInvincibility(f32);

// TODO:
// 1. Player HP UI
// 2. Monster damage
// 3. (Monster HP UI)
// 4. Weapon switching
// 5. First weapon (rectangles) NOTE: What about projectiles?
// 6. Shooting + Aiming
// 7. Rolling
// 8. Score
fn main() {
    App::new()
        // Resources
        .insert_resource(WindowDescriptor {
            title: "Cthulhu-Strike 1.6".to_string(),
            ..default()
        })
        // Startup Systems
        .add_startup_system(setup_camera)
        .add_startup_system(setup_ui)
        .add_startup_system(spawn_player)
        // Systems
        .add_system(player_movement)
        .add_system(camera_lock.after(player_movement))
        .add_system(enemy_movement.after(player_movement))
        .add_system(update_health_ui.after(enemy_movement))
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
        .register_inspectable::<Speed>()
        .register_inspectable::<Health>()
        .run();
}

// ###############
// ### Systems ###
// ###############
fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera)
        .insert(Name::new("MainCamera"));
}

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
        .insert(Name::new("Player"))
        .insert(Health(PLAYER_HEALTH))
        .insert(Speed(PLAYER_SPEED));
}

fn player_movement(
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Speed), With<Player>>,
) {
    let (mut player_transform, player_speed) = player_query.single_mut();
    let mut target_point: Vec3 = player_transform.translation;

    if keys.pressed(KeyCode::W) {
        target_point.y += 1.0;
    }
    if keys.pressed(KeyCode::A) {
        target_point.x -= 1.0;
    }
    if keys.pressed(KeyCode::S) {
        target_point.y -= 1.0;
    }
    if keys.pressed(KeyCode::D) {
        target_point.x += 1.0;
    }

    let player_movement_vector =
        scaled_vector_between_points(&player_transform.translation, &target_point, **player_speed);

    player_transform.translation.x += player_movement_vector.x;
    player_transform.translation.y += player_movement_vector.y;
}

fn update_health_ui(
    player_health: Query<&Health, With<Player>>,
    mut health_text: Query<&mut Text, With<HealthText>>,
) {
    let player_health = player_health.single();
    let mut health_text = health_text.single_mut();

    health_text.sections[0].value = format!("Health: {}", **player_health);
}

fn camera_lock(
    player_transform: Query<&Transform, With<Player>>,
    mut camera_transform: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    let player_transform = player_transform.single();
    let mut camera_transform = camera_transform.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

// Enemy Systems
fn spawn_enemy(mut commands: Commands, translation: Vec3) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: ENEMY_COLOR,
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
        .insert(Name::new("Enemy"))
        .insert(Health(ENEMY_HEALTH))
        .insert(Speed(ENEMY_SPEED));
}

// TODO: Add logic so that enemies can't be inside another enemy or the player
fn enemy_movement(
    player_transform: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Transform, &Speed), (With<Enemy>, Without<Player>)>,
) {
    let player_translation = player_transform.single().translation;

    for (mut enemy_transform, enemy_speed) in enemy_query.iter_mut() {
        let enemy_player_vector = scaled_vector_between_points(
            &enemy_transform.translation,
            &player_translation,
            **enemy_speed,
        );

        enemy_transform.translation.x += enemy_player_vector.x;
        enemy_transform.translation.y += enemy_player_vector.y;
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

// ########################
// ### Helper Functions ###
// ########################
// TODO: Consider reducing the return value to (f32, f32)
fn scaled_vector_between_points(from: &Vec3, to: &Vec3, scale: f32) -> Vec3 {
    let delta_x = to.x - from.x;
    let delta_y = to.y - from.y;

    Vec3::new(delta_x, delta_y, 0.0).normalize_or_zero() * scale
}
