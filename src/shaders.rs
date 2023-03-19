
use macroquad::prelude::*;

pub const DEFAULT_FRAGMENT_SHADER: &'static str = "\
#version 100
precision lowp float;

// verlet object properties
uniform lowp vec2 pos_old;
uniform lowp vec2 pos_curr;
uniform lowp vec2 acceleration;
uniform lowp float radius;
uniform lowp float temperature;

// rendering coordinates (object-space and screen-space)
varying vec2 uv;
varying vec3 pos;

void main() {
    gl_FragColor = vec4(
        -pos.x-pos.y,
        0.5-abs(pos.x)+pos.y/2.0,
        pos.x-pos.y,
        1.0
    );
}
";

pub const FIRE_FRAGMENT_SHADER: &'static str = "\
#version 100
precision lowp float;

// verlet object properties
uniform lowp vec2 pos_old;
uniform lowp vec2 pos_curr;
uniform lowp vec2 acceleration;
uniform lowp float radius;
uniform lowp float temperature;

// rendering coordinates (object-space and screen-space)
varying vec2 uv;
varying vec3 pos;

void main() {
    gl_FragColor = vec4(
        temperature * 3.00,
        temperature * 0.60,
        temperature * 0.25,
        1.0
    );
}
";

pub const WATER_FRAGMENT_SHADER: &'static str = "\
#version 100
precision lowp float;

// verlet object properties
uniform lowp vec2 pos_old;
uniform lowp vec2 pos_curr;
uniform lowp vec2 acceleration;
uniform lowp float radius;
uniform lowp float temperature;

// rendering coordinates (object-space and screen-space)
varying vec2 uv;
varying vec3 pos;

void main() {
    float vel = distance(pos_old, pos_curr);
    gl_FragColor = vec4(
        vec2(vel * 0.3),
        vel * 0.7,
        1.0
    );
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

pub struct ShaderContext {
    // shader uniforms usage
    pub use_uniform_pos_old: bool,
    pub use_uniform_pos_curr: bool,
    pub use_uniform_acceleration: bool,
    pub use_uniform_radius: bool,
    pub use_uniform_temperature: bool,

    // ui variables
    pub use_shaders: bool,
    pub auto_reload_shaders: bool,

    // shader variables
    pub fragment_shader: String,
    pub vertex_shader: String,
    pipeline_params: PipelineParams,
    uniforms: Vec<(String, UniformType)>,
    pub material: Material,
    pub shader_error: String,
}

impl ShaderContext {
    pub fn default() -> Self{
        let pipeline_params: PipelineParams = PipelineParams::default();
        let uniforms: Vec<(String, UniformType)> = vec![
            ("pos_curr".to_string(), UniformType::Float2),
            ("pos_old".to_string(), UniformType::Float2),
            ("acceleration".to_string(), UniformType::Float2),
            ("radius".to_string(), UniformType::Float1),
            ("temperature".to_string(), UniformType::Float1),
        ];
        Self {
            // shader uniforms usage
            use_uniform_pos_old: false,
            use_uniform_pos_curr: false,
            use_uniform_acceleration: false,
            use_uniform_radius: false,
            use_uniform_temperature: false,

            // ui variables
            use_shaders: false,
            auto_reload_shaders: true,

            // shader variables
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