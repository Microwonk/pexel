#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    pixellation_intensity: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: vec3<f32>
#endif
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // Pixellation strength
    let pixel_size = settings.pixellation_intensity;
    // Calculate the pixelated UV coordinates
    let uv = floor(in.uv * vec2<f32>(1.0 / pixel_size)) / vec2<f32>(1.0 / pixel_size);
    // Sample the color at the pixelated UV coordinates
    let pixelated = textureSample(screen_texture, texture_sampler, uv);
    return pixelated;
}