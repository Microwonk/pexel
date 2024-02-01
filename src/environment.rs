use bevy::{pbr::ExtendedMaterial, prelude::*};

use crate::{shaders::QuantizationExtension, GameState};

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_environment);
    }
}

fn spawn_environment(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut c_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, QuantizationExtension>>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // sphere
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(
            Mesh::try_from(shape::Icosphere {
                radius: 1.0,
                subdivisions: 5,
            })
            .unwrap(),
        ),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: c_materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: Color::PINK,
                ..default()
            },
            extension: QuantizationExtension { quantize_steps: 10 },
        }),
        ..default()
    });

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(
            Mesh::try_from(shape::Icosphere {
                radius: 1.0,
                subdivisions: 5,
            })
            .unwrap(),
        ),
        transform: Transform::from_xyz(5.0, 5., 5.0),
        material: c_materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: Color::YELLOW_GREEN,
                emissive: Color::YELLOW_GREEN,
                reflectance: 0.9,
                ..default()
            },
            extension: QuantizationExtension { quantize_steps: 10 },
        }),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(50.0).into()),
        material: materials.add(Color::SILVER.into()),
        ..default()
    });
}
