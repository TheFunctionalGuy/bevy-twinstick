use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

pub struct InspectionPlugin;

impl Plugin for InspectionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_inspectable::<Health>()
            .register_inspectable::<Invincible>()
            .register_inspectable::<Speed>()
            .register_inspectable::<Damage>()
            .register_inspectable::<CurrentAmmo>()
            .register_inspectable::<MaximumAmmo>()
            .register_inspectable::<Reloading>();
    }
}

// World Objects
#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Enemy;
#[derive(Component)]
pub struct Weapon;

// Stats
#[derive(Component, Deref, DerefMut, Inspectable)]
pub struct Health(pub i32);
#[derive(Component, Deref, DerefMut, Inspectable)]
pub struct Invincible(pub bool);
#[derive(Component, Deref, DerefMut)]
pub struct InvincibilityTimer(pub Timer);
#[derive(Component, Deref, DerefMut, Inspectable)]
pub struct Speed(pub f32);
#[derive(Component, Deref, DerefMut, Inspectable)]
pub struct Damage(pub i32);
#[derive(Component, Deref, DerefMut, Inspectable)]
pub struct CurrentAmmo(pub u32);
#[derive(Component, Deref, DerefMut, Inspectable)]
pub struct MaximumAmmo(pub u32);
#[derive(Component, Deref, DerefMut)]
pub struct FireDelayTimer(pub Timer);
#[derive(Component, Deref, DerefMut)]
pub struct ReloadTimer(pub Timer);
#[derive(Component, Deref, DerefMut, Inspectable)]
pub struct Reloading(pub bool);

// Properties
#[derive(Component)]
pub struct HealthText;
#[derive(Component)]
pub struct WeaponText;
#[derive(Component)]
pub struct AmmoText;
#[derive(Component)]
pub struct EnemyText;
#[derive(Component)]
pub struct MainCamera;
