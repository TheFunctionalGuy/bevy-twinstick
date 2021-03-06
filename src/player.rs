use bevy::prelude::*;

use crate::{
    components::{Health, InvincibilityTimer, Invincible, MainCamera, Player, Speed},
    util::VectorMath,
};

// Constants
const PLAYER_COLOR: Color = Color::BLUE;
const PLAYER_SPEED: f32 = 120.0;
const PLAYER_HEALTH: i32 = 5;
const PLAYER_INVINCIBILITY_TIME: f32 = 2.0;

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
        .insert(Invincible(false))
        .insert(InvincibilityTimer(Timer::from_seconds(
            PLAYER_INVINCIBILITY_TIME,
            false,
        )))
        .insert(Speed(PLAYER_SPEED));
}

// TODO: Consider moving all input handling to separate plugin
pub fn player_movement(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Speed), With<Player>>,
) {
    let (mut player_transform, player_speed) = player_query.single_mut();
    let mut target_point: Vec2 = player_transform.translation.truncate();

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

    let player_movement_vector = player_transform
        .translation
        .truncate()
        .scaled_vector_to(&target_point, **player_speed * time.delta_seconds());

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
