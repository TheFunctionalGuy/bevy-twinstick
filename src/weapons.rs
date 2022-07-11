use bevy::prelude::*;

use crate::components::{
    CurrentAmmo, Damage, FireDelayTimer, MaximumAmmo, ReloadTimer, Reloading, Weapon,
};

// Constants
// (name, damage, ammo, delay, reload)
const WEAPONS: [(&str, i32, u32, f32, f32); 5] = [
    ("Pistols", 10_i32, 30_u32, 0.3_f32, 2.0_f32),
    // Reload time is meant per pellet
    ("Shotgun", 30_i32, 7_u32, 1.0_f32, 0.75_f32),
    ("AssaultRifle", 15_i32, 30_u32, 0.1_f32, 1.5_f32),
    ("RocketLauncher", 50_i32, 1_u32, 1.5_f32, 2.5_f32),
    // TODO: Laser has special ammo system
    ("Laser", 10_i32, 30_u32, 0.1_f32, 1.5_f32),
];

// Resource
#[derive(Default, Deref, DerefMut)]
struct SelectedWeapon(Option<Entity>);

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
            .add_system(handle_weapon_input)
            .insert_resource(SelectedWeapon::default())
            .insert_resource(Weapons::default());
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
// TODO: Add delay between shoots
fn handle_weapon_input(
    mouse_buttons: Res<Input<MouseButton>>,
    selected_weapon: Res<SelectedWeapon>,
    mut weapon_ammo: Query<&mut CurrentAmmo>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        if let Some(weapon_ent) = **selected_weapon {
            if let Ok(mut current_ammo) = weapon_ammo.get_mut(weapon_ent) {
                if **current_ammo > 0 {
                    **current_ammo -= 1;
                } else {
                    println!("Weapon empty!");
                }
            }
        }
    }
}
