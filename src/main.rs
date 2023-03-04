
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
    // set scaling ?
    // set bounce on constraint similar to collision resolution
    // finish and credit tutorial (link chain stuff)
    // improve UI

    let mut rng: ThreadRng = thread_rng();
    let mut spawn_count: usize = 10;

    let fps: f64 = 60.0;
    let mut last_frame = get_time();

    let mut solver: Solver = Solver::new();

    loop {
        //logic
        let now: f64 = get_time();
        let frame_time: f64 = now - last_frame;
        if frame_time >= 1.0 / fps {
            solver.update_with_substep(frame_time as f32, 8);
            last_frame = now;
        }

        //input
        // if is_mouse_button_pressed(MouseButton::Left) {
        //     let (x, y): (f32, f32) = mouse_position();
        //     let obj: VerletObject = VerletObject::new(x, y);
        //     solver.verlet_objects.push(obj);
        // }
        // for touch in touches() {
        //     let macroquad::prelude::Vec2{x, y} = touch.position;
        //     let obj: VerletObject = VerletObject::new(x, y);
        //     solver.verlet_objects.push(obj);
        // }

        // rendering
        clear_background(BLACK);
        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 20., 20., WHITE);
        draw_text(format!("Objects: {}", solver.verlet_objects.len()).as_str(), 0., 40., 20., WHITE);
        let mut text_index = 3.0;
        for verlet_object in &solver.verlet_objects {
            let Vec2{x, y} = verlet_object.position_current;
            let r = verlet_object.radius;
            draw_circle(x, y, r, Color::new(1.0, 1.0, 1.0, 0.3));
            if !(0.0..screen_width()).contains(&x) {
                draw_text(format!("OOB: [{}, {}]", x, y).as_str(), 0., 20. * text_index, 20., RED);
                text_index += 1.0;
            }
        }
        
        // ui
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Spawn Verlet object")
                .show(egui_ctx, |ui| {
                    ui.add(egui::Slider::new(&mut spawn_count, 10..=1000));
                    if ui.button("Spawn").clicked() {
                        for _ in 0..spawn_count {
                            let pos: Vec2 = Vec2 {
                                x: rng.gen_range(50.0..screen_width()-50.0),
                                y: rng.gen_range(50.0..screen_height()-50.0),
                            };
                            let obj: VerletObject = VerletObject {
                                position_current: pos.clone(),
                                position_old: pos.clone(),
                                acceleration: Vec2::zero(),
                                radius: 10.0,
                            };
                            solver.verlet_objects.push(obj);
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
