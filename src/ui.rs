
use crate::{context::{self, Context}, syntax_highlighting::CodeTheme};
use macroquad::prelude::*;
use ::rand::{rngs::ThreadRng, thread_rng, Rng};

pub const RIGHT: f32 = 0.0;
pub const DOWN: f32 = 90.0;
pub const LEFT: f32 = 180.0;
pub const UP: f32 = 270.0;

pub struct Windows {
    pub controls: bool,
    pub stats: bool,
    pub shaders: bool,
    pub rules: bool,
}

impl Windows {
    pub fn new() -> Self {
        Self {
            controls: false,
            stats: false,
            shaders: false,
            rules: false,
        }
    }
}

pub fn render(context: &mut Context, windows: &mut Windows) {
    egui_macroquad::ui(|egui_ctx| {
        egui::Window::new("SimWindows")
            .collapsible(true)
            .show(egui_ctx, |ui| {
                ui.checkbox(&mut windows.controls, "Controls");
                ui.checkbox(&mut windows.stats, "Stats");
                ui.checkbox(&mut windows.shaders, "Shaders");
                ui.checkbox(&mut windows.rules, "Rules");
            });
        egui::Window::new("Controls")
            .open(&mut windows.controls)
            .show(egui_ctx, |ui| {
                controls(ui, context);
            });
        egui::Window::new("Stats")
            .open(&mut windows.stats)
            .show(egui_ctx, |ui| {
                stats(ui, context, get_fps() as f32);
            });
        egui::Window::new("Shaders")
            .open(&mut windows.shaders)
            .default_size((600.0, 500.0))
            .show(egui_ctx, |ui| {
                code_editor(ui, context);
            });
        egui::Window::new("Rules")
            .open(&mut windows.rules)
            .show(egui_ctx, |ui| {
                rules(ui, context);
            });
    });

    egui_macroquad::draw();
}

pub fn code_editor(ui: &mut egui::Ui, context: &mut Context) {
    if cfg!(target_arch = "wasm32") && cfg!(target_os = "unknown") {
        ui.checkbox(&mut context.use_shaders, "Use shaders (WARNING!)")
            .on_hover_ui(|ui| {
                    ui.label("(Warning: Don't paste (Ctrl+V) anything into these text boxes else the website crashes)");
            });
    } else {
        ui.checkbox(&mut context.use_shaders, "Use shaders");
    }

    ui.separator();
    ui.horizontal(|ui| {
        ui.add_enabled_ui(context.use_shaders, |ui| {
            if ui.button("Reload shaders").clicked() {
                context.reload_shaders();
            }
            if ui.button("Reset to default").clicked() {
                context.vertex_shader = context::DEFAULT_VERTEX_SHADER.to_string();
                context.fragment_shader = context::DEFAULT_FRAGMENT_SHADER.to_string();
            }
            if ui.checkbox(&mut context.auto_reload_shaders, "Reload automatically on change").clicked() {
                if context.auto_reload_shaders {
                    context.reload_shaders();
                }
            }
        });
    });

    ui.separator();

    let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
        let mut layout_job =
            crate::syntax_highlighting::highlight(ui.ctx(), &CodeTheme::dark(), string, "glsl");
        layout_job.wrap.max_width = wrap_width;
        ui.fonts(|f| f.layout_job(layout_job))
    };
    
    ui.collapsing("Fragment shader", |ui| {
        ui.add_enabled_ui(context.use_shaders, |ui| {
            egui::ScrollArea::vertical()
            .id_source("fragment_scroll_area")
            .show(ui, |ui| {
                if ui.add(
                    egui::TextEdit::multiline(&mut context.fragment_shader)
                        .code_editor()
                        .desired_rows(20)
                        .desired_width(f32::INFINITY)
                        .layouter(&mut layouter)
                ).changed() {
                    if context.auto_reload_shaders {
                        context.reload_shaders();
                    }
                }
            });
        });
    });

    ui.collapsing("Vertex shader", |ui| {
        ui.add_enabled_ui(context.use_shaders, |ui| {
            egui::ScrollArea::vertical()
                .id_source("vertext_scroll_area")
                .show(ui, |ui| {
                    if ui.add(
                        egui::TextEdit::multiline(&mut context.vertex_shader)
                            .code_editor()
                            .desired_rows(20)
                            .desired_width(f32::INFINITY)
                            .layouter(&mut layouter)
                    ).changed() {
                        if context.auto_reload_shaders {
                            context.reload_shaders();
                        }
                    }
                });
        });
    });

    ui.separator();
    ui.label("Shader compilation error message:");
    ui.add_enabled(false, 
        egui::TextEdit::multiline(&mut context.shader_error)
            .code_editor()
            .desired_width(f32::INFINITY)
    );
}

pub fn stats(ui: &mut egui::Ui, context: &mut Context, fps: f32) {
    ui.label("(SFPS stands for simulation frames per second)");
    ui.label(format!("FPS: {:.02} ({:.02}ms)", fps, 1000.0 / fps));
    ui.label(format!("SFPS: {:.02} ({:.02}ms)", 1.0 / context.last_sim_frame_time, context.last_sim_frame_time * 1000.0));
    ui.label(format!("Objects: {}", context.solver.verlet_objects.len()));
    ui.label(format!("Cell size: {} Grid size: [{}, {}]", context.solver.cell_size, context.solver.cell_grid[0].len(), context.solver.cell_grid.len()));

    // substep size
    ui.separator();
    ui.add(
        egui::Slider::new(&mut context.sim_substeps, 1..=32).text("Substep count")
    );
    if ui.button("Reset substep count").clicked() {
        context.sim_substeps = 8;
    }

    // target max fps to prevent 4k+ fps goofs
    ui.separator();
    ui.checkbox(&mut context.sfps_target_enforced, "Use target SFPS");
    ui.horizontal(|ui| {
        ui.add_enabled(
            context.sfps_target_enforced,
            egui::Slider::new(&mut context.sfps_target, 30.0..=120.0)
        );
        ui.add_enabled_ui(context.sfps_target_enforced, |ui| {
            ui.label(format!("FPS ({:.02}ms)", 1000.0 / context.sfps_target));
        });
    });
    if ui.button("Reset target frame time").clicked() {
        context.sfps_target = 60.0;
    };

    // delta time values passed into solver.update()
    ui.separator();
    ui.label("Forcibly modify values passed to solver.update()");

    // max SFPS
    ui.checkbox(&mut context.sfps_max_enforced, "Enforce max SFPS");
    ui.horizontal(|ui| {
        ui.add_enabled(
            context.sfps_max_enforced,
            egui::Slider::new(&mut context.sfps_max, 30.0..=120.0)
        );
        ui.add_enabled_ui(context.sfps_max_enforced, |ui| {
            ui.label(format!("FPS ({:.02}ms)", 1000.0 / context.sfps_max));
        });
    });
    if ui.button("Reset forced max frame time").clicked() {
        context.sfps_max = 60.0;
    };

    // min SFPS
    ui.checkbox(&mut context.sfps_min_enforced, "Enforce min SFPS");
    ui.horizontal(|ui| {
        ui.add_enabled(
            context.sfps_min_enforced,
            egui::Slider::new(&mut context.sfps_min, 30.0..=120.0)
        );
        ui.add_enabled_ui(context.sfps_min_enforced, |ui| {
            ui.label(format!("FPS ({:.02}ms)", 1000.0 / context.sfps_min));
        });
    });
    if ui.button("Reset forced min frame time").clicked() {
        context.sfps_min = 60.0;
    };
}

pub fn controls(ui: &mut egui::Ui, context: &mut Context) {
    ui.checkbox(&mut context.accept_direct_controls, "Enable manual controls");
    if context.accept_direct_controls {
        ui.label("Add objects with scroll down");
        ui.label("Remove objects with scroll up");
    }

    ui.separator();
    ui.label("Object management");
    ui.add(egui::Slider::new(&mut context.spawn_count, 100..=1000).text("Count"));
    ui.add(egui::Slider::new(&mut context.solver.spawn_radius, 1.0..=50.0).text("Radius"));
    ui.horizontal(|ui| {
        if ui.button("Spawn").clicked() {
            context.solver.spawn_count(context.spawn_count);
        }
        if ui.button("Remove").clicked() {
            context.solver.remove_count(context.spawn_count);
        }
        if ui.button("Clear").clicked() {
            context.solver.clear();
        }
    });

    ui.separator();
    ui.label("Accelerate all objects");
    let mut direction_string: String = String::new();
    let tmp_direction: f32 = context.shake_direction % 360.0;
    if tmp_direction > RIGHT && tmp_direction < LEFT {
        direction_string += "Down";
    } else if tmp_direction > LEFT {
        direction_string += "Up"
    }
    if tmp_direction > DOWN && tmp_direction < UP {
        if !direction_string.is_empty() {
            direction_string += "-";
        }
        direction_string += "Left";
    } else if tmp_direction < DOWN || tmp_direction > UP {
        if !direction_string.is_empty() {
            direction_string += "-";
        }
        direction_string += "Right"
    }
    ui.add(egui::Slider::new(&mut context.shake_intensity, 100_000.0..=1_000_000.0).text("Intensitiy"));
    ui.add(egui::Slider::new(&mut context.shake_direction, 0.0..=360.0)
        .text(format!("Direction ({direction_string})"))
        .custom_formatter(|p, _| {format!("{p}??").to_string()}));
    ui.horizontal(|ui| {
        if ui.button("Right").clicked() {
            context.shake_direction = RIGHT;
        }
        if ui.button("Down").clicked() {
            context.shake_direction = DOWN;
        }
        if ui.button("Left").clicked() {
            context.shake_direction = LEFT;
        }
        if ui.button("Up").clicked() {
            context.shake_direction = UP;
        }
    });
    let mut rng: ThreadRng = thread_rng();
    ui.horizontal(|ui| {
        if ui.button("Randomize direction").clicked() {
            context.shake_direction = rng.gen_range(0..360) as f32;
        }
        ui.checkbox(&mut context.shake_auto_random, "After each acceleration");
    });
    if ui.button("Accelerate").clicked() {
        context.solver.accelerate_all(context.shake_intensity, context.shake_direction.to_radians());
        if context.shake_auto_random {
            context.shake_direction = rng.gen_range(0..360) as f32;
        }
    }

    ui.separator();
    ui.label("Stabilization");
    ui.horizontal(|ui| {
        if ui.button("Stabilize").clicked() {
            context.solver.stabilize();
        }
        ui.checkbox(&mut context.solver.stabilize_on_oob, "On OOB");
        ui.checkbox(&mut context.solver.stabilize_on_spawn, "On spawn");
    });

    ui.separator();
    ui.collapsing("Safety measures", |ui| {
        ui.separator();
        ui.add(egui::Slider::new(&mut context.solver.spawn_safety_radius_factor, 0.0..=2.0).text("Safe spawn radius factor"));
        ui.add(egui::Slider::new(&mut context.solver.spawn_safety_iterations, 1..=100).text("Safe spawn iterations"));
    });
}

pub fn rules(ui: &mut egui::Ui, context: &mut Context) {
    ui.label("Gravity");
    ui.add(
        egui::Slider::new(&mut context.solver.gravity.x, 0.0..=10_000.0).text("X axis")
    );
    ui.add(
        egui::Slider::new(&mut context.solver.gravity.y, 0.0..=10_000.0).text("Y axis")
    );
    ui.horizontal(|ui| {
        if ui.button("Default").clicked() {
            context.solver.gravity.x = 0.0;
            context.solver.gravity.y = 1_000.0;
        }
        if ui.button("Zero").clicked() {
            context.solver.gravity.x = 0.0;
            context.solver.gravity.y = 0.0;
        }
    });


    ui.separator();
    ui.label("Contraint enforcement");
    ui.checkbox(&mut context.solver.apply_constraint_bottom, "Apply bottom constraint");
    ui.checkbox(&mut context.solver.apply_constraint_top, "Apply top constraint");
    ui.checkbox(&mut context.solver.apply_constraint_left, "Apply left constraint");
    ui.checkbox(&mut context.solver.apply_constraint_right, "Apply right constraint");

    ui.separator();
    ui.label("Object count enforcement");
    ui.checkbox(&mut context.solver.max_object_count_enforced, "Enforce max object count");
    ui.add_enabled(
        context.solver.max_object_count_enforced,
        egui::Slider::new(&mut context.solver.max_object_count, 0..=5000)
    );
    ui.checkbox(&mut context.solver.min_object_count_enforced, "Enforce min object count");
    ui.add_enabled(
        context.solver.min_object_count_enforced,
        egui::Slider::new(&mut context.solver.min_object_count, 0..=5000)
    );
}
