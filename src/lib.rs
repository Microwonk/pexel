#![allow(clippy::type_complexity)]

mod camera;
mod environment;
mod loading;
mod shaders;

use environment::EnvironmentPlugin;
use shaders::ShadersPlugin;
use smooth_bevy_cameras::controllers::fps::FpsCameraPlugin;
use smooth_bevy_cameras::LookTransformPlugin;

use crate::camera::CameraPlugin;
use crate::loading::LoadingPlugin;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins((
            ShadersPlugin,
            LookTransformPlugin,
            FpsCameraPlugin::default(),
            LoadingPlugin,
            CameraPlugin,
            EnvironmentPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}
