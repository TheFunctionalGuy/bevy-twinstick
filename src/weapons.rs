use bevy::prelude::*;

use crate::components::{CurrentAmmo, Damage, MaximumAmmo, ReloadTime, Weapon};

// Constants

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
        app.add_startup_system(spawn_weapon)
            .add_system(handle_weapon_input);
    }
}

// Systems
fn spawn_weapon(mut commands: Commands) {
    commands.spawn_bundle(WeaponBundle {
        name: Name::new("Pistols"),
        damage: Damage(10),
        maximum_ammo: MaximumAmmo(30),
        current_ammo: CurrentAmmo(30),
        reload_time: ReloadTime(1.5),
        _weapon: Weapon,
    });
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
