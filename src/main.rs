
use macroquad::prelude::*;
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

#[macroquad::main("mq-verlet")]
async fn main() {
    // fire:
    //      this is about as good as I can get it, maybe try removing temperature based on how much the object traveled since last frame
    //      add math equation parser with variable input and relevant UI for goofing
    // resistance
    // learn how to use shaders more effectively (water shader in the book, passing in whole textures etc)
    // game rules (circle constaint (with adjust point gravity), constaint condition (temperature, bounce))
    // debug for android wasm
    // time warp thingy, including complete stop (divide frame_time before passing to update)
    // if last sim frame time < target frame time: disable target frame time
    // better defaults for web
    // (unlocked sim frame time, enabled lower 60 sfps max limit)
    //      #[cfg(target_arch = "wasm32")]
    //      #[cfg(target_os = "unknown")]
    // simplify highlighting (maybe remove enum_map dependency)
    // make everything (or generic?) f64 and compare
    // shaders (fft, fire?)
    // documentation
    // spawned from this: 3d version
    // parametrize stuff
    //      what color to what color gradient based on what scale of velocity
    //      temperature of particles (heat loss, temp to vel, color with scale)
    //      velocity to radius
    //      constraint type (window, circle, combinations)
    // add presets (maybe need automation):
    //      mixer (cw or ccw shake timed accordingly)
    //      0 grav bowling/pool like stuff 
    //      auto shaking (with looping over stuff and bpm/settable delay per shake)
    //      rain chaos (min 1001, max 1000)
    //      stable preset for window resize playing
    //      stable density showcase (big go up, shake it up a little)
    //      max objects at different sizes with stable fps
    //      bubbling away (slowly replacing big ones with small ones)
    // approximate object limits before FPS drops:
    //      naive: 1600
    //      cellularized: 3300
    //      cell (heap fixed): 5000

    let mut context: Context = Context::default();
    let mut windows: Windows = Windows::new();
    let mut last_frame: f64 = get_time();

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
                let (x, y): (f32, f32) = mouse_position();
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
