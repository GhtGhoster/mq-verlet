
use macroquad::prelude::*;
use ::rand::{thread_rng, rngs::ThreadRng, Rng};
use solver::Solver;
use verlet::VerletObject;
use vector::Vec2;

mod vector;
mod verlet;
mod solver;

#[macroquad::main("mq-verlet")]
async fn main() {
    // object limits before FPS drops:
    //  naive: 1600
    //  cellularized: 3300
    //  cell (heap fixed): TODO

    let mut rng: ThreadRng = thread_rng();

    let mut spawn_radius: f32 = 10.0;
    let mut spawn_count: usize = 100;
    let mut auto_stabilize: bool = true;
    let mut spawn_stabilize: bool = false;
    let mut spawn_safety_radius_factor: f32 = 1.0;
    let mut spawn_safety_iterations: usize = 100;
    let mut min_object_count: usize = 0;
    let mut ensure_min_object_count: bool = false;
    let mut max_object_count: usize = 1500; // old max limit before FPS drops
    let mut ensure_max_object_count: bool = false;

    let fps: f64 = 60.0;
    let mut last_frame = get_time();
    let mut max_frame_time: f64 = 0.0;

    let mut solver: Solver = Solver::new();

    loop {
        // logic
        let now: f64 = get_time();
        let mut frame_time: f64 = now - last_frame;
        if frame_time >= 1.0 / fps {
            frame_time = frame_time.min(0.1);
            solver.update_with_substep(frame_time as f32, 8);
            last_frame = now;
            max_frame_time = frame_time;
        }

        // ui logic
        if ensure_min_object_count {
            while solver.verlet_objects.len() < min_object_count {
                let pos: Vec2 = Vec2 {
                    x: rng.gen_range(spawn_radius..screen_width()-spawn_radius),
                    y: rng.gen_range(spawn_radius..screen_height()-spawn_radius),
                };
                let obj: VerletObject = VerletObject {
                    position_current: pos.clone(),
                    position_old: pos.clone(),
                    acceleration: Vec2::zero(),
                    radius: spawn_radius,
                };
                solver.push(obj);
            }
        }
        if ensure_max_object_count {
            while solver.verlet_objects.len() > max_object_count {
                solver.remove(0);
            }
        }
        if auto_stabilize {
            for i in 0..solver.verlet_objects.len() {
                let Vec2{x, y} = solver.verlet_objects[i].position_current;
                let r = solver.verlet_objects[i].radius;
                if !(-r..screen_width()+r).contains(&x) || !(-r..screen_height()+r).contains(&y) {
                    if auto_stabilize {
                        solver.stabilize();
                        break;
                    }
                }
            }
        }

        // rendering
        clear_background(BLACK);
        let mut text_index = 1.0;
        for verlet_object in &solver.verlet_objects {
            let Vec2{x, y} = verlet_object.position_current;
            let r = verlet_object.radius;
            if !(-r..screen_width()+r).contains(&x) || !(-r..screen_height()+r).contains(&y) {
                draw_text(format!("OOB: [{}, {}]", x, y).as_str(), 0., 20. * text_index, 20., RED);
                text_index += 1.0;
            } else {
                draw_circle(x, y, r, Color::new(1.0, 1.0, 1.0, 0.3));
            }
        }
        
        // ui
        if mouse_wheel().1 < 0.0 {
            let (x, y): (f32, f32) = mouse_position();
            let pos: Vec2 = Vec2 {x, y};
            solver.spawn(pos, spawn_radius);
        }
        if mouse_wheel().1 > 0.0 {
            let (x, y): (f32, f32) = mouse_position();
            let pos: Vec2 = Vec2 {x, y};
            solver.remove_pos(pos);
        }
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Spawn Verlet object")
                .show(egui_ctx, |ui| {
                    // header
                    ui.label("Add objects with scroll down, remove with scroll up");
                    ui.collapsing("Stats:", |ui| {
                        ui.label(format!("FPS: {}", get_fps()));
                        ui.label(format!("SimFPS: {:.2}", (1.0 / max_frame_time)));
                        ui.label(format!("SimFrame time: {:.2}ms", (max_frame_time * 1000.0)));
                        ui.label(format!("Objects: {}", solver.verlet_objects.len()));
                    });

                    // main options
                    ui.separator();
                    ui.add(egui::Slider::new(&mut spawn_count, 100..=1000).text("Count"));
                    ui.add(egui::Slider::new(&mut spawn_radius, 1.0..=50.0).text("Radius"));
                    ui.horizontal(|ui| {
                        if ui.button("Spawn").clicked() {
                            solver.spawn_count(spawn_count, spawn_radius, spawn_safety_iterations, spawn_safety_radius_factor, spawn_stabilize);
                        }
                        if ui.button("Remove").clicked() {
                            solver.remove_count(spawn_count);
                        }
                        if ui.button("Clear").clicked() {
                            solver.clear();
                        }
                    });

                    // safety measures
                    ui.separator();
                    ui.collapsing("Safety measures", |ui| {
                        ui.horizontal(|ui| {
                            if ui.button("Stabilize").clicked() {
                                solver.stabilize();
                            }
                            ui.checkbox(&mut auto_stabilize, "Automatic");
                            ui.checkbox(&mut spawn_stabilize, "On spawn");
                        });

                        ui.separator();
                        ui.add(egui::Slider::new(&mut spawn_safety_radius_factor, 0.0..=2.0).text("Safe spawn radius"));
                        ui.add(egui::Slider::new(&mut spawn_safety_iterations, 1..=100).text("Safe spawn iterations"));

                        ui.separator();
                        ui.add(egui::Slider::new(&mut min_object_count, 0..=20000).text("Minimum object count"));
                        ui.checkbox(&mut ensure_min_object_count, "Ensure min object count");
                        ui.add(egui::Slider::new(&mut max_object_count, 0..=20000).text("Maximum object count"));
                        ui.checkbox(&mut ensure_max_object_count, "Ensure max object count");
                    });
                }
            );
        });
        egui_macroquad::draw();
        next_frame().await
    }
}
