use bevy::prelude::*;

// Constants
const PLAYER_COLOR: Color = Color::BLUE;

// Components
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Health(i32);

// TODO:
// 1. Spawn player (blue tile)
// 2. Player movement
// 3. Monster spawner (red tiles)
// 4. First weapon (rectangles) NOTE: What about projectiles?
// 5. Weapon switching
// 6. Shooting + Aiming
fn main() {
    App::new()
        // Startup Systems
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_player)
        // Systems
        .add_system(hello_world_system)
        // Plugins
        .add_plugins(DefaultPlugins)
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

// fn spawn_player(commmands: Commands) {
//     commmands.s
// }

fn hello_world_system() {
    println!("Hello, World!");
}
