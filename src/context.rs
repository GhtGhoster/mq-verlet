
use macroquad::prelude::*;

use crate::solver::Solver;

pub const DEFAULT_FRAGMENT_SHADER: &'static str = "\
#version 100
precision lowp float;

uniform lowp vec4 kekw;

varying vec2 uv;
varying vec3 pos;

void main() {
    gl_FragColor = vec4(1.0-(pos.y+pos.x)/2.0, 1.0-pos.x, 1.0-pos.y, 1.0);
}
";

pub const DEFAULT_VERTEX_SHADER: &'static str = "\
#version 100
precision lowp float;

attribute vec3 position;
attribute vec2 texcoord;

varying vec2 uv;
varying vec3 pos;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    uv = texcoord;
    pos = gl_Position.xyz;
}
";

pub struct Context {
    pub solver: Solver,

    // simulation fps - waiting until another simulation frame is supposed to happen
    pub sfps_target: f64,
    pub sfps_target_enforced: bool,

    // simulation fps - max of frame time of this value and calculated frame time
    pub sfps_min: f64,
    pub sfps_min_enforced: bool,

    // simulation fps - min of frame time of this value and calculated frame time
    pub sfps_max: f64,
    pub sfps_max_enforced: bool,

    // simulation substeps
    pub sim_substeps: usize,

    // actual resulting simulation frame time
    pub last_sim_frame_time: f32,

    pub accept_direct_controls: bool,

    // interaction variables
    pub spawn_count: usize,

    pub shake_auto_random: bool,
    pub shake_intensity: f32,
    pub shake_direction: f32,

    // shader variables
    pub use_shaders: bool,
    pub auto_reload_shaders: bool,

    pub fragment_shader: String,
    pub vertex_shader: String,
    pipeline_params: PipelineParams,
    uniforms: Vec<(String, UniformType)>,
    pub material: Material,
    pub shader_error: String,

}

impl Context {
    pub fn default() -> Self {
        let pipeline_params: PipelineParams = PipelineParams::default();
        let uniforms: Vec<(String, UniformType)> = vec![];
        Self {
            solver: Solver::new(),

            sfps_target: 60.0,
            sfps_target_enforced: !(cfg!(target_arch = "wasm32") && cfg!(target_os = "unknown")),
        
            sfps_min: 60.0,
            sfps_min_enforced: false,
        
            sfps_max: 60.0,
            sfps_max_enforced: false,
        
            sim_substeps: 8,
        
            last_sim_frame_time: 0.0,

            accept_direct_controls: true,

            spawn_count: 100,

            shake_auto_random: false,
            shake_intensity: 100_000.0,
            shake_direction: 90.0,
            
            use_shaders: false,
            auto_reload_shaders: true,

            fragment_shader: DEFAULT_FRAGMENT_SHADER.to_string(),
            vertex_shader: DEFAULT_VERTEX_SHADER.to_string(),
            pipeline_params,
            uniforms: uniforms.clone(),
            material: load_material(
                &DEFAULT_VERTEX_SHADER.to_string(),
                &DEFAULT_FRAGMENT_SHADER.to_string(),
                MaterialParams {
                    uniforms: uniforms.clone(),
                    pipeline_params,
                    textures: vec![],
                },
            ).unwrap(),
            shader_error: String::new(),
        }
    }

    pub fn reload_shaders(&mut self) {
        match load_material(
            self.vertex_shader.as_str(),
            self.fragment_shader.as_str(),
            MaterialParams {
                uniforms: self.uniforms.clone(),
                pipeline_params: self.pipeline_params,
                textures: vec![],
            },
        ) {
            Ok(mat) => {
                self.material.delete();
                self.material = mat;
                self.shader_error = String::new();
            },
            Err(error) => {
                self.shader_error = error.to_string();
            },
        }
    }
}
