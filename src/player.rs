use bevy::prelude::*;

use crate::{
    components::{Health, MainCamera, Player, Speed},
    util::scaled_vector_between_points,
};

// Constants
const PLAYER_COLOR: Color = Color::BLUE;
const PLAYER_SPEED: f32 = 2.0;
const PLAYER_HEALTH: i32 = 5;

// Plugin
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(camera_lock.after(player_movement));
    }
}

// Systems
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

// TODO: Consider moving all input handling to separate plugin
pub fn player_movement(
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

fn camera_lock(
    player_transform: Query<&Transform, With<Player>>,
    mut camera_transform: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    let player_transform = player_transform.single();
    let mut camera_transform = camera_transform.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}
