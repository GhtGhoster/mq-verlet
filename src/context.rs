
use macroquad::prelude::*;

use crate::solver::Solver;
use crate::shaders::{ShaderContext, FIRE_FRAGMENT_SHADER, WATER_FRAGMENT_SHADER};
use crate::vector;

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
    
    // presets
    pub current_preset_name: String,
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

            current_preset_name: "Default".to_string(),
        }
    }

    pub fn reset(&mut self) {
        self.solver = Solver::new();
        self.sfps_target = 60.0;
        self.sfps_target_enforced = !(cfg!(target_arch = "wasm32") && cfg!(target_os = "unknown"));
        self.sfps_min = 60.0;
        self.sfps_min_enforced = false;
        self.sfps_max = 60.0;
        self.sfps_max_enforced = false;
        self.sim_substeps = 8;
        self.accept_direct_controls = true;
        self.random_direct_controls = true;
        self.spawn_count = 100;
        self.shake_auto_random = false;
        self.shake_intensity = 100_000.0;
        self.shake_direction = 90.0;
        self.shader_context.material.delete();
        self.shader_context = ShaderContext::default();
        self.current_preset_name = "Default".to_string();
    }

    pub fn stable_sixty_web_preset(&mut self) {
        self.reset();
        self.sfps_max_enforced = true;
        self.sfps_min_enforced = true;
    }

    pub fn stable_thirty_web_preset(&mut self) {
        self.stable_sixty_web_preset();
        self.sfps_max = 30.0;
        self.sfps_min = 30.0;
    }

    pub fn fire_preset_one(&mut self) {
        self.solver = Solver::new();
        self.solver.accelerate_on_temperature = true;
        self.solver.temperature_acceleration_power = 2.0;
        self.solver.gravity.y = 9_500.0;
        self.solver.apply_temperature_bottom = 0.1;
        self.solver.min_object_count = 1_500;
        self.solver.min_object_count_enforced = true;
        self.shader_context.material.delete();
        self.shader_context = ShaderContext::default();
        self.shader_context.use_shaders = true;
        self.shader_context.use_uniform_temperature = true;
        self.shader_context.fragment_shader = FIRE_FRAGMENT_SHADER.to_string();
        self.shader_context.reload_shaders();
        self.current_preset_name = "Fire 1".to_string();
    }

    pub fn fire_preset_two(&mut self) {
        self.fire_preset_one();
        self.solver.gravity.y = 6_000.0;
        self.solver.temperature_acceleration_power = 3.0;
        self.current_preset_name = "Fire 2".to_string();
    }

    pub fn rain_preset(&mut self) {
        self.solver = Solver::new();
        self.solver.min_object_count = 30;
        self.solver.min_object_count_enforced = true;
        self.solver.apply_constraint_bottom = false;
        self.solver.apply_constraint_top = false;
        self.solver.apply_constraint_left = false;
        self.solver.apply_constraint_right = false;
        self.solver.stabilize_on_oob = false;
        self.shader_context.use_shaders = true;
        self.shader_context.use_uniform_pos_curr = true;
        self.shader_context.use_uniform_pos_old = true;
        self.shader_context.fragment_shader = WATER_FRAGMENT_SHADER.to_string();
        self.shader_context.reload_shaders();
        self.current_preset_name = "Rain".to_string();
    }

    pub fn bowling_pool_preset(&mut self) {
        self.reset();
        self.solver.stabilize_on_oob = false;
        self.solver.apply_constraint_bottom = false;
        self.solver.apply_constraint_top = false;
        self.solver.apply_constraint_left = false;
        self.solver.apply_constraint_right = false;
        self.solver.spawn_radius = 50.0;
        self.solver.gravity = vector::Vec2::zero();

        let mid_y = screen_height() * 0.5;
        let horizontal_spacing = 60.0 * 3f32.sqrt();

        // cue/bowling ball
        self.solver.spawn(vector::Vec2{x: 400.0, y: mid_y});

        // pins
        self.solver.spawn(vector::Vec2{x: screen_width() - 200.0, y: mid_y + 60.0});
        self.solver.spawn(vector::Vec2{x: screen_width() - 200.0, y: mid_y - 60.0});
        self.solver.spawn(vector::Vec2{x: screen_width() - 200.0, y: mid_y + 180.0});
        self.solver.spawn(vector::Vec2{x: screen_width() - 200.0, y: mid_y - 180.0});

        self.solver.spawn(vector::Vec2{x: screen_width() - 200.0 - horizontal_spacing, y: mid_y});
        self.solver.spawn(vector::Vec2{x: screen_width() - 200.0 - horizontal_spacing, y: mid_y + 120.0});
        self.solver.spawn(vector::Vec2{x: screen_width() - 200.0 - horizontal_spacing, y: mid_y - 120.0});

        self.solver.spawn(vector::Vec2{x: screen_width() - 200.0 - horizontal_spacing*2.0, y: mid_y + 60.0});
        self.solver.spawn(vector::Vec2{x: screen_width() - 200.0 - horizontal_spacing*2.0, y: mid_y - 60.0});

        self.solver.spawn(vector::Vec2{x: screen_width() - 200.0 - horizontal_spacing*3.0, y: mid_y});
    }
}
