use std::f32::consts::PI;

use bevy::{core::FixedTimestep, prelude::*};
use rand::random;

use crate::{
    components::{Enemy, Health, Player, Speed},
    player::player_movement,
    util::scaled_vector_between_points,
};

// Constants
const ENEMY_COLOR: Color = Color::RED;
const ENEMY_SPEED: f32 = 1.5;
const ENEMY_HEALTH: i32 = 25;

const INITIAL_ENEMY_DISTANCE: f32 = 750.0;
const MAXIMUM_ENEMY_COUNT: usize = 10;

// Plugin
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(enemy_spawner),
        )
        .add_system(enemy_movement.after(player_movement));
    }
}

// Systems
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
// TODO: Make independent from FPS
pub fn enemy_movement(
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
