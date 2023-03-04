
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
    // finish and credit tutorial (link chain stuff)
    // optimize for >~1600 objects (credit)

    let mut rng: ThreadRng = thread_rng();

    let mut spawn_radius: f32 = 10.0;
    let mut spawn_count: usize = 10;
    let mut min_object_count: usize = 0;
    let mut ensure_min_object_count: bool = false;
    let mut max_object_count: usize = 1500;
    let mut ensure_max_object_count: bool = false;

    let fps: f64 = 60.0;
    let mut last_frame = get_time();
    let mut max_frame_time: f64 = 0.0;

    let mut solver: Solver = Solver::new();

    loop {
        //logic
        let now: f64 = get_time();
        let mut frame_time: f64 = now - last_frame;
        if frame_time >= 1.0 / fps {
            frame_time = frame_time.min(0.1);
            solver.update_with_substep(frame_time as f32, 8);
            last_frame = now;
            max_frame_time = frame_time;
        }

        // rendering
        clear_background(BLACK);
        let mut text_index = 1.0;
        for verlet_object in &solver.verlet_objects {
            let Vec2{x, y} = verlet_object.position_current;
            let r = verlet_object.radius;
            if !(-r..screen_width()+r).contains(&x) || !(-r..screen_height()+r).contains(&y){
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
            let obj: VerletObject = VerletObject {
                position_current: pos.clone(),
                position_old: pos.clone(),
                acceleration: Vec2::zero(),
                radius: spawn_radius,
            };
            solver.verlet_objects.push(obj);
        }
        if mouse_wheel().1 > 0.0 {
            let (x, y): (f32, f32) = mouse_position();
            let pos: Vec2 = Vec2 {x, y};
            for i in (0..solver.verlet_objects.len()).rev() {
                if (solver.verlet_objects[i].position_current - pos).len() < solver.verlet_objects[i].radius {
                    solver.verlet_objects.remove(i);
                }
            }
        }
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Spawn Verlet object")
                .show(egui_ctx, |ui| {
                    ui.label("Add objects with scroll down, remove with scroll up");
                    ui.label(format!("FPS: {}", get_fps()));
                    ui.label(format!("SimFrame time: {:.2}ms", (max_frame_time * 1000.0)));
                    ui.label(format!("SimFPS: {:.2}", (1.0 / max_frame_time)));
                    ui.label(format!("Objects: {}", solver.verlet_objects.len()));
                    ui.add(egui::Slider::new(&mut spawn_count, 10..=1000).text("Count"));
                    ui.add(egui::Slider::new(&mut spawn_radius, 1.0..=50.0).text("Radius"));
                    if ui.button("Spawn").clicked() {
                        for _ in 0..spawn_count {
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
                            solver.verlet_objects.push(obj);
                        }
                    }
                    ui.add(egui::Slider::new(&mut min_object_count, 0..=2000).text("Minimum object count"));
                    ui.checkbox(&mut ensure_min_object_count, "Ensure min object count");
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
                            solver.verlet_objects.push(obj);
                        }
                    }
                    ui.add(egui::Slider::new(&mut max_object_count, 0..=2000).text("Maximum object count"));
                    ui.checkbox(&mut ensure_max_object_count, "Ensure max object count");
                    if ensure_max_object_count {
                        while solver.verlet_objects.len() > max_object_count {
                            solver.verlet_objects.pop();
                        }
                    }
                    if ui.button("Clear").clicked() {
                        solver.verlet_objects.clear();
                    }
                }
            );
        });
        egui_macroquad::draw();
        next_frame().await
    }
}
