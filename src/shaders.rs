mod materials;
mod post_processing;

use bevy::app::Plugin;
pub(crate) use materials::*;
pub(crate) use post_processing::*;

pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((PostProcessPlugin, MaterialsPlugin));
    }
}
