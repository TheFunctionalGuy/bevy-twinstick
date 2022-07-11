use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

pub struct InspectionPlugin;

impl Plugin for InspectionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_inspectable::<Health>()
            .register_inspectable::<Speed>();
    }
}

// World Objects
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

// Stats
#[derive(Component, Deref, DerefMut, Inspectable)]
pub struct Health(pub i32);

#[derive(Component, Deref, DerefMut, Inspectable)]
pub struct Speed(pub f32);

// Properties
#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct MainCamera;
