
use macroquad::prelude::*;
use crate::context::Context;
use crate::vector::Vec2;

pub fn render(context: &mut Context) {
    // basics
    clear_background(BLACK);
    let mut oob_text: Vec<String> = vec![];

    // shaders
    if context.use_shaders {
        set_camera(&Camera2D {
            zoom: vec2(2.0/screen_width(), -2.0/screen_height()),
            target: vec2(screen_width()/2.0, screen_height()/2.0),
            ..Default::default()
        });
        gl_use_material(context.material);
    }

    // object rendering
    for verlet_object in &context.solver.verlet_objects {
        let Vec2{x, y} = verlet_object.position_current;
        let r = verlet_object.radius;
        if !(-r..screen_width()+r).contains(&x) || !(-r..screen_height()+r).contains(&y) {
            oob_text.push(format!("OOB: [{}, {}]", x, y));
        } else {

            if context.use_shaders {
                context.material.set_uniform("kekw", vec4(1.0, 1.0, 1.0, 1.0));
            }

            draw_circle(x, y, r, Color::new(1.0, 1.0, 1.0, 0.4));
        }
    }

    // shader clean up
    if context.use_shaders {
        set_default_camera();
        gl_use_default_material();
    }

    // oob object rendering (text)
    for (i, text) in oob_text.iter().enumerate() {
        draw_text(text, 0.0, 20.0 * i as f32, 20., RED);
    }
}
