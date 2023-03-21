
use macroquad::prelude::*;
use ::rand::{rngs::ThreadRng, thread_rng, Rng};
use vector::Vec2;
use context::Context;
use ui::Windows;

mod vector;
mod verlet;
mod solver;
mod ui;
mod context;
mod render;
mod syntax_highlighting;
mod shaders;

#[macroquad::main("mq-verlet")]
async fn main() {
    let mut context: Context = Context::default();
    let mut windows: Windows = Windows::new();
    let mut last_frame: f64 = get_time();
    let mut rng: ThreadRng = thread_rng();

    loop {
        // logic
        let now: f64 = get_time();
        let mut frame_time: f64 = now - last_frame;
        if !context.sfps_target_enforced || frame_time >= 1.0 / context.sfps_target {
            if context.sfps_min_enforced {
                frame_time = frame_time.min(1.0 / context.sfps_min);
            }
            if context.sfps_max_enforced {
                frame_time = frame_time.max(1.0 / context.sfps_max);
            }
            context.solver.update_with_substep(frame_time as f32, context.sim_substeps);
            context.last_sim_frame_time = frame_time as f32;
            last_frame = now;
        }
        
        // direct input
        if context.accept_direct_controls {
            if mouse_wheel().1 < 0.0 {
                let (mut x, mut y): (f32, f32) = mouse_position();
                if context.random_direct_controls {
                    x += rng.gen_range(-0.5..0.5);
                    y += rng.gen_range(-0.5..0.5);
                }
                let pos: Vec2 = Vec2 {x, y};
                context.solver.spawn(pos);
            }
            if mouse_wheel().1 > 0.0 {
                let (x, y): (f32, f32) = mouse_position();
                let pos: Vec2 = Vec2 {x, y};
                context.solver.remove_pos(pos);
            }
        }

        // simulation rendering
        render::render(&mut context);

        // ui rendering
        ui::render(&mut context, &mut windows);

        next_frame().await
    }
}
