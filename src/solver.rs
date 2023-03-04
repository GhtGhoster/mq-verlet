
use crate::{vector::Vec2, verlet::VerletObject};
use macroquad::prelude::{screen_width, screen_height};

pub struct Solver {
    pub gravity: Vec2,
    pub verlet_objects: Vec<VerletObject>,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            gravity: Vec2 {
                x: 0.0,
                y: 1000.0,
            },
            verlet_objects: vec![],
        }
    }

    pub fn push(&mut self, obj: VerletObject) {
        self.verlet_objects.push(obj);
    }

    pub fn remove(&mut self, obj_index: usize) {
        self.verlet_objects.remove(obj_index);
    }

    pub fn clear(&mut self) {
        self.verlet_objects.clear();
    }

    pub fn update_with_substep(&mut self, dt: f32, substebs: usize) {
        let sub_dt: f32 = dt / substebs as f32;
        for _ in 0..substebs {
            self.update(sub_dt);
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.apply_gravity();
        self.apply_constraint();
        self.solve_collisions();
        self.update_positions(dt);
        self.remove_oob_objs();
    }

    pub fn update_positions(&mut self, dt: f32) {
        for obj in self.verlet_objects.iter_mut() {
            obj.update_position(dt);
        }
    }

    pub fn apply_gravity(&mut self) {
        for obj in self.verlet_objects.iter_mut() {
            obj.accelerate(self.gravity);
        }
    }

    pub fn apply_constraint(&mut self) {
        for obj in self.verlet_objects.iter_mut() {
            if obj.position_current.x < obj.radius {
                obj.position_current.x = obj.radius;
            } else if obj.position_current.x > screen_width() - obj.radius {
                obj.position_current.x = screen_width() - obj.radius;
            }
            if obj.position_current.y < obj.radius {
                obj.position_current.y = obj.radius;
            } else if obj.position_current.y > screen_height() - obj.radius {
                obj.position_current.y = screen_height() - obj.radius;
            }
        }
    }

    pub fn solve_collisions(&mut self) {
        for i in 0..self.verlet_objects.len() {
            for k in i+1..self.verlet_objects.len() {
                let collision_axis: Vec2 = self.verlet_objects[i].position_current - self.verlet_objects[k].position_current;
                let dist: f32 = collision_axis.len();
                let radii: f32 = self.verlet_objects[i].radius + self.verlet_objects[k].radius;
                if dist < radii {
                    let n: Vec2 = collision_axis / dist;
                    let delta: f32 = radii - dist;
                    self.verlet_objects[i].position_current += n * 0.5 * delta;
                    self.verlet_objects[k].position_current -= n * 0.5 * delta;
                }
            }
        }
    }

    pub fn remove_oob_objs(&mut self) {
        for i in (0..self.verlet_objects.len()).rev() {
            if self.verlet_objects[i].position_old.x.is_nan() || self.verlet_objects[i].position_old.y.is_nan() {
                self.verlet_objects.remove(i);
            }
        }
    }
}