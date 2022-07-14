use std::f32::consts::FRAC_PI_6;

use bevy::prelude::*;

use crate::{
    components::{
        CurrentAmmo, Damage, Enemy, FireDelayTimer, Health, MaximumAmmo, Player, ReloadTimer,
        Reloading, Weapon,
    },
    mouse::MousePosition,
    util::VectorMath,
};

// Constants
// (name, damage, RANGE, ammo, delay, reload)
const WEAPONS: [(&str, i32, u32, f32, f32); 5] = [
    ("Pistols", 10_i32, 30_u32, 0.3_f32, 2.0_f32),
    // Reload time is meant per pellet
    ("Shotgun", 30_i32, 7_u32, 1.0_f32, 0.75_f32),
    ("AssaultRifle", 15_i32, 30_u32, 0.1_f32, 1.5_f32),
    ("RocketLauncher", 50_i32, 1_u32, 1.5_f32, 2.5_f32),
    // TODO: Laser has special ammo system
    ("Laser", 10_i32, 30_u32, 0.1_f32, 1.5_f32),
];

// Resources
#[derive(Default, Deref, DerefMut)]
pub struct SelectedWeapon(Option<Entity>);

#[derive(Default, Deref, DerefMut)]
struct Weapons(Vec<Entity>);

// Bundle
#[derive(Bundle)]
struct WeaponBundle {
    name: Name,
    damage: Damage,
    maximum_ammo: MaximumAmmo,
    current_ammo: CurrentAmmo,
    fire_delay_timer: FireDelayTimer,
    reload_timer: ReloadTimer,
    reloading: Reloading,
    _weapon: Weapon,
}

// Plugin
pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_weapons)
            .add_system(select_weapon)
            .add_system(shoot_weapon)
            .add_system(reload_weapon.after(shoot_weapon))
            .insert_resource(SelectedWeapon::default())
            .insert_resource(Weapons::default())
            .insert_resource(MousePosition::default());
    }
}

// Systems
fn spawn_weapons(
    mut commands: Commands,
    mut weapons: ResMut<Weapons>,
    mut selected_weapon: ResMut<SelectedWeapon>,
) {
    // Add all weapons to weapon resource
    for (name, damage, ammo, fire_delay, reload_time) in WEAPONS {
        weapons.push(
            commands
                .spawn_bundle(WeaponBundle {
                    name: Name::new(name),
                    damage: Damage(damage),
                    maximum_ammo: MaximumAmmo(ammo),
                    current_ammo: CurrentAmmo(ammo),
                    fire_delay_timer: FireDelayTimer(Timer::from_seconds(fire_delay, false)),
                    reload_timer: ReloadTimer(Timer::from_seconds(reload_time, false)),
                    reloading: Reloading(false),
                    _weapon: Weapon,
                })
                .id(),
        );

        **selected_weapon = Some(weapons[0]);
    }
}

// TODO: Add fire mode
pub fn shoot_weapon(
    mut commands: Commands,
    mouse_buttons: Res<Input<MouseButton>>,
    time: Res<Time>,
    selected_weapon: Res<SelectedWeapon>,
    mouse_position: Res<MousePosition>,
    mut weapon_query: Query<(
        &mut CurrentAmmo,
        &mut FireDelayTimer,
        &mut Reloading,
        &mut ReloadTimer,
        &Damage,
    )>,
    player_transform: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(Entity, &Transform, &mut Health), (With<Enemy>, Without<Player>)>,
) {
    if let Some(weapon_ent) = **selected_weapon {
        if let Ok((
            mut current_ammo,
            mut fire_delay_timer,
            mut reloading,
            mut reload_timer,
            weapon_damage,
        )) = weapon_query.get_mut(weapon_ent)
        {
            fire_delay_timer.tick(time.delta());

            if mouse_buttons.just_pressed(MouseButton::Left)
                && fire_delay_timer.finished()
                && !**reloading
            {
                **current_ammo -= 1;
                fire_delay_timer.reset();

                // Calculate damage
                let player_position = player_transform.single().translation.truncate();
                let scaled_target_vector = player_position.scaled_vector_to(&mouse_position, 500.0);
                let b = player_position + scaled_target_vector.rotated_by(FRAC_PI_6);
                let c = player_position + scaled_target_vector.rotated_by(-FRAC_PI_6);

                for (enemy_ent, enemy_transform, mut enemy_health) in enemy_query.iter_mut() {
                    let enemy_position = enemy_transform.translation.truncate();

                    if enemy_position.is_in_triangle(&player_position, &b, &c) {
                        if **weapon_damage >= **enemy_health {
                            commands.entity(enemy_ent).despawn();
                        } else {
                            **enemy_health -= **weapon_damage;
                        }
                    }
                }

                // Auto-reload
                if **current_ammo == 0 {
                    **reloading = true;
                    reload_timer.reset();
                }
            }
        }
    }
}

fn reload_weapon(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    selected_weapon: Res<SelectedWeapon>,
    mut weapon_ammo: Query<(
        &MaximumAmmo,
        &mut CurrentAmmo,
        &mut Reloading,
        &mut ReloadTimer,
    )>,
) {
    if let Some(weapon_ent) = **selected_weapon {
        if let Ok((maximum_ammo, mut current_ammo, mut reloading, mut reload_timer)) =
            weapon_ammo.get_mut(weapon_ent)
        {
            if **reloading {
                reload_timer.tick(time.delta());

                if reload_timer.just_finished() {
                    **current_ammo = **maximum_ammo;
                    **reloading = false;
                }
            } else if keys.pressed(KeyCode::R) {
                **reloading = true;
                reload_timer.reset();
            }
        }
    }
}

fn select_weapon(
    keys: Res<Input<KeyCode>>,
    weapons: Res<Weapons>,
    mut selected_weapon: ResMut<SelectedWeapon>,
) {
    if keys.pressed(KeyCode::Key1) {
        **selected_weapon = Some(weapons[0]);
    }
    if keys.pressed(KeyCode::Key2) {
        **selected_weapon = Some(weapons[1]);
    }
    if keys.pressed(KeyCode::Key3) {
        **selected_weapon = Some(weapons[2]);
    }
    if keys.pressed(KeyCode::Key4) {
        **selected_weapon = Some(weapons[3]);
    }
    if keys.pressed(KeyCode::Key5) {
        **selected_weapon = Some(weapons[4]);
    }
}
