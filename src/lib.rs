#![allow(clippy::type_complexity)]

mod camera;
mod environment;
mod loading;
mod post_processing;

use smooth_bevy_cameras::controllers::fps::FpsCameraPlugin;
use smooth_bevy_cameras::LookTransformPlugin;

use crate::camera::CameraPlugin;
use crate::loading::LoadingPlugin;
pub(crate) use crate::post_processing::PostProcessPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use environment::EnvironmentPlugin;

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
            LookTransformPlugin,
            FpsCameraPlugin::default(),
            LoadingPlugin,
            CameraPlugin,
            EnvironmentPlugin,
            PostProcessPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}
