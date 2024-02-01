use crate::{shaders::PostProcessSettings, GameState};
use bevy::prelude::*;
use smooth_bevy_cameras::controllers::fps::{FpsCameraBundle, FpsCameraController};

pub struct CameraPlugin;

#[derive(Component)]
pub struct Player;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_camera);
    }
}

#[allow(clippy::needless_update)] // because of webgl
fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(FpsCameraBundle::new(
            FpsCameraController {
                mouse_rotate_sensitivity: Vec2::splat(2.0),
                enabled: true,
                translate_sensitivity: 10.,
                smoothing_weight: 0.5,
            },
            Vec3::new(-2.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ))
        .insert(PostProcessSettings {
            pixellation_intensity: 0.003,
            chrom_intensity: 5.,
            ..default()
        });
}
