use bevy::prelude::*;

use crate::components::{CurrentAmmo, MaximumAmmo, ReloadTime, Weapon};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_weapon);
    }
}

fn spawn_weapon(mut commands: Commands) {
    commands
        .spawn()
        .insert(Weapon)
        .insert(Name::new("Pistols"))
        .insert(MaximumAmmo(30))
        .insert(CurrentAmmo(30))
        .insert(ReloadTime(1.5));
}
