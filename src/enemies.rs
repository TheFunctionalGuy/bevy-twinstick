use std::f32::consts::PI;

use bevy::{core::FixedTimestep, prelude::*, sprite::collide_aabb::collide};
use rand::random;

use crate::{
    components::{Enemy, Health, InvincibilityTimer, Invincible, Player, Speed},
    player::player_movement,
    util::VectorMath,
};

// Constants
const ENEMY_COLOR: Color = Color::RED;
const ENEMY_SPEED: f32 = 90.0;
const ENEMY_HEALTH: i32 = 25;

const INITIAL_ENEMY_DISTANCE: f32 = 750.0;
const MAXIMUM_ENEMY_COUNT: usize = 10;
const ENEMY_SPAWN_DELAY: f64 = 1.0;

// Plugin
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(ENEMY_SPAWN_DELAY))
                .with_system(enemy_spawner),
        )
        .add_system(enemy_movement.after(player_movement))
        .add_system(enemy_damage.after(enemy_movement));
    }
}

// Systems
fn enemy_spawner(
    mut commands: Commands,
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

        spawn_enemy(&mut commands, enemy_translation);
    }
}

fn spawn_enemy(commands: &mut Commands, translation: Vec3) {
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
pub fn enemy_movement(
    time: Res<Time>,
    player_transform: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Transform, &Speed), (With<Enemy>, Without<Player>)>,
) {
    let player_position = player_transform.single().translation.truncate();

    for (mut enemy_transform, enemy_speed) in enemy_query.iter_mut() {
        let enemy_player_vector = enemy_transform
            .translation
            .truncate()
            .scaled_vector_to(&player_position, **enemy_speed * time.delta_seconds());

        enemy_transform.translation.x += enemy_player_vector.x;
        enemy_transform.translation.y += enemy_player_vector.y;
    }
}

fn enemy_damage(
    time: Res<Time>,
    mut player_query: Query<
        (
            &Transform,
            &mut Health,
            &mut Invincible,
            &mut InvincibilityTimer,
        ),
        With<Player>,
    >,
    enemy_transforms: Query<&Transform, (With<Enemy>, Without<Player>)>,
) {
    let (player_transform, mut player_health, mut invincible, mut invincibility_timer) =
        player_query.single_mut();

    if !**invincible {
        for enemy_transform in enemy_transforms.iter() {
            if collide(
                player_transform.translation,
                Vec2::new(30.0, 30.0),
                enemy_transform.translation,
                Vec2::new(30.0, 30.0),
            )
            .is_some()
            {
                **player_health -= 1;
                **invincible = true;
                invincibility_timer.reset();

                break;
            }
        }
    } else {
        invincibility_timer.tick(time.delta());

        if invincibility_timer.just_finished() {
            **invincible = false;
        }
    }
}
