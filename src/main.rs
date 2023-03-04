
use macroquad::prelude::*;
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

    let fps: f64 = 60.0;
    let mut last_frame = get_time();

    let mut solver: Solver = Solver::new();

    loop {
        while get_time() - last_frame < 1.0 / fps {
        }
        last_frame = get_time();
        // ui
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Spawn Verlet object")
                .show(egui_ctx, |ui| {
                    if ui.button("Spawn!").clicked() {
                        solver.verlet_objects.push(VerletObject::new(screen_width(), screen_height()));
                    }
                }
            );
        });

        solver.update(get_frame_time());

        // rendering
        clear_background(BLACK);
        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 20., 20., WHITE);
        for verlet_object in &solver.verlet_objects {
            let Vec2{x, y} = verlet_object.position_current;
            let r = verlet_object.radius;
            draw_circle(x, y, r, WHITE);
        }

        egui_macroquad::draw();
        next_frame().await
    }
}
