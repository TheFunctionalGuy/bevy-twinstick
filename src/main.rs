use bevy::prelude::*;

// Components
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Health;

fn main() {
    App::new()
        // Startup Systems
        .add_startup_system(setup_camera)
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

// fn spawn_player(commmands: Commands) {
//     commmands.s
// }

fn hello_world_system() {
    println!("Hello, World!");
}
