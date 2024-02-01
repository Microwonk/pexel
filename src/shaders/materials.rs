use bevy::{
    app::{App, Plugin},
    asset::Asset,
    pbr::{ExtendedMaterial, MaterialExtension, MaterialPlugin, StandardMaterial},
    prelude::default,
    reflect::Reflect,
    render::render_resource::{AsBindGroup, ShaderRef},
};

pub struct MaterialsPlugin;

impl Plugin for MaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, QuantizationExtension>,
        > {
            prepass_enabled: false,
            ..default()
        });
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct QuantizationExtension {
    // leave space for the original materials uniforms
    #[uniform(100)]
    pub quantize_steps: u32,
}

impl MaterialExtension for QuantizationExtension {
    fn fragment_shader() -> ShaderRef {
        "shaders/quantization_mat.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/quantization_mat.wgsl".into()
    }
}
