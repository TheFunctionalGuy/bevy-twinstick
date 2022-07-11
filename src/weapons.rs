use bevy::prelude::*;

use crate::components::{CurrentAmmo, Damage, MaximumAmmo, ReloadTime, Weapon};

// Constants
const WEAPONS: [(&str, i32, u32, f32); 5] = [
    ("Pistols", 10_i32, 30_u32, 2.0_f32),
    // Reload time is meant per pellet
    ("Shotgun", 30_i32, 7_u32, 0.75_f32),
    ("AssaultRifle", 15_i32, 30_u32, 1.5_f32),
    ("RocketLauncher", 50_i32, 1_u32, 2.5_f32),
    // TODO: Laser has special ammo system
    ("Laser", 10_i32, 30_u32, 1.5_f32),
];

// Bundle
#[derive(Bundle)]
struct WeaponBundle {
    name: Name,
    damage: Damage,
    maximum_ammo: MaximumAmmo,
    current_ammo: CurrentAmmo,
    reload_time: ReloadTime,
    _weapon: Weapon,
}

// Plugin
pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_weapons)
            .add_system(handle_weapon_input);
    }
}

// Systems
fn spawn_weapons(mut commands: Commands) {
    for (name, damage, ammo, reload_time) in WEAPONS {
        commands.spawn_bundle(WeaponBundle {
            name: Name::new(name),
            damage: Damage(damage),
            maximum_ammo: MaximumAmmo(ammo),
            current_ammo: CurrentAmmo(ammo),
            reload_time: ReloadTime(reload_time),
            _weapon: Weapon,
        });
    }
}

fn handle_weapon_input(
    mouse_buttons: Res<Input<MouseButton>>,
    weapon_ammo: Query<&mut CurrentAmmo>,
) {
    if mouse_buttons.pressed(MouseButton::Left) {
        shoot_weapon(weapon_ammo);
    }
}

// TODO: Shoot only selected weapon
// TODO: Add fire mode
// TODO: Add delay between shoots
fn shoot_weapon(mut weapon_ammo: Query<&mut CurrentAmmo>) {
    for mut current_ammo in weapon_ammo.iter_mut() {
        if **current_ammo > 0 {
            **current_ammo -= 1;
        } else {
            println!("Weapon empty!");
        }
    }
}
