
use macroquad::prelude::*;

use crate::solver::Solver;
use crate::shaders::ShaderContext;

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

    // manual controls
    pub accept_direct_controls: bool,
    pub random_direct_controls: bool,

    // interaction variables
    pub spawn_count: usize,

    pub shake_auto_random: bool,
    pub shake_intensity: f32,
    pub shake_direction: f32,

    // shaders
    pub shader_context: ShaderContext,
}

impl Context {
    pub fn default() -> Self {
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
            random_direct_controls: true,

            spawn_count: 100,

            shake_auto_random: false,
            shake_intensity: 100_000.0,
            shake_direction: 90.0,

            shader_context: ShaderContext::default(),
        }
    }
}
