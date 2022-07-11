use bevy::prelude::*;

use crate::{
    components::{AmmoText, CurrentAmmo, Health, HealthText, Player, Weapon, WeaponText},
    enemies::enemy_movement,
    weapons::SelectedWeapon,
};

// Constants
const TEXT_COLOR: Color = Color::WHITE;

// Plugin
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui)
            .add_system(update_health.after(enemy_movement))
            .add_system(update_selected_weapon)
            .add_system(update_current_ammo);
    }
}

// Systems
fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: Rect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::FlexStart,
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
            color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect {
                            left: Val::Px(5.0),
                            right: Val::Px(5.0),
                            ..default()
                        },
                        ..default()
                    },
                    text: Text::with_section(
                        "Health: %",
                        TextStyle {
                            font: font.clone(),
                            font_size: 30.0,
                            color: TEXT_COLOR,
                        },
                        Default::default(),
                    ),
                    ..default()
                })
                .insert(HealthText);

            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect {
                            left: Val::Px(5.0),
                            right: Val::Px(5.0),
                            ..default()
                        },
                        ..default()
                    },
                    text: Text::with_section(
                        "Weapon: %",
                        TextStyle {
                            font: font.clone(),
                            font_size: 30.0,
                            color: TEXT_COLOR,
                        },
                        Default::default(),
                    ),
                    ..default()
                })
                .insert(WeaponText);

            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect {
                            left: Val::Px(5.0),
                            right: Val::Px(5.0),
                            ..default()
                        },
                        ..default()
                    },
                    text: Text::with_section(
                        "Ammo: %",
                        TextStyle {
                            font: font.clone(),
                            font_size: 30.0,
                            color: TEXT_COLOR,
                        },
                        Default::default(),
                    ),
                    ..default()
                })
                .insert(AmmoText);
        });
}

// TODO: Refactor
fn update_health(
    player_health: Query<&Health, With<Player>>,
    mut health_text: Query<&mut Text, With<HealthText>>,
) {
    let player_health = player_health.single();
    let mut health_text = health_text.single_mut();

    health_text.sections[0].value = format!("Health: {}", **player_health);
}

// TODO: Consider merging weapon ui functions
fn update_selected_weapon(
    selected_weapon: Res<SelectedWeapon>,
    weapon_names: Query<&Name, With<Weapon>>,
    mut weapon_text: Query<&mut Text, With<WeaponText>>,
) {
    let mut weapon_text = weapon_text.single_mut();

    if let Some(weapon_ent) = **selected_weapon {
        if let Ok(weapon_name) = weapon_names.get(weapon_ent) {
            weapon_text.sections[0].value = format!("Weapon: {}", weapon_name);
        }
    }
}

fn update_current_ammo(
    selected_weapon: Res<SelectedWeapon>,
    weapon_ammos: Query<&CurrentAmmo, With<Weapon>>,
    mut ammo_text: Query<&mut Text, With<AmmoText>>,
) {
    let mut ammo_text = ammo_text.single_mut();

    if let Some(weapon_ent) = **selected_weapon {
        if let Ok(weapon_ammo) = weapon_ammos.get(weapon_ent) {
            ammo_text.sections[0].value = format!("Weapon: {}", **weapon_ammo);
        }
    }
}
