
use crate::{vector::Vec2, verlet::VerletObject};
use macroquad::prelude::{screen_width, screen_height};
use rand::{rngs::ThreadRng, thread_rng, Rng};

pub struct Solver {
    pub gravity: Vec2,
    pub verlet_objects: Vec<VerletObject>,
    pub cell_size: f32,
    // pub cell_grid: Vec<Vec<Vec<usize>>>,
}

impl Solver {
    pub fn new() -> Self {
        // let grid_width: usize = screen_width().ceil() as usize;
        // let grid_height: usize = screen_height().ceil() as usize;
        // let mut grid: Vec<Vec<Vec<usize>>> = Vec::with_capacity(grid_height);
        // for i in 0..grid_height {
        //     grid.push(Vec::with_capacity(grid_width));
        //     for _ in 0..grid_width {
        //         grid[i].push(vec![]);
        //     }
        // }
        Self {
            gravity: Vec2 {
                x: 0.0,
                y: 1000.0,
            },
            verlet_objects: vec![],
            cell_size: 100.0, // optimize default after removing constant heap allocation
            // cell_grid: grid,
        }
    }

    pub fn push(&mut self, obj: VerletObject) {
        // optimize cell size (factor to prevent "popcorn effect")
        self.cell_size = self.cell_size.max(obj.radius * 2.5);
        self.verlet_objects.push(obj);
    }

    pub fn remove(&mut self, obj_index: usize) {
        self.verlet_objects.remove(obj_index);
    }

    pub fn remove_count(&mut self, obj_count: usize) {
        for _ in 0..obj_count {
            if self.verlet_objects.len() == 0 {break;}
            self.remove(0);
        }
    }

    pub fn remove_pos(&mut self, pos: Vec2) {
        for i in (0..self.verlet_objects.len()).rev() {
            if (self.verlet_objects[i].position_current - pos).len() < self.verlet_objects[i].radius {
                self.remove(i);
            }
        }
    }

    pub fn clear(&mut self) {
        self.verlet_objects.clear();
    }
    
    pub fn stabilize(&mut self) {
        for i in 0..self.verlet_objects.len() {
            self.verlet_objects[i].position_old = self.verlet_objects[i].position_current;
        }
    }

    pub fn spawn(&mut self, pos: Vec2, radius: f32) {
        self.push(VerletObject::new(pos, radius));
    }

    pub fn spawn_count(
        &mut self,
        spawn_count: usize,
        spawn_radius: f32,
        spawn_safety_iterations: usize,
        spawn_safety_radius_factor: f32,
        spawn_stabilize: bool
    ) {
        let mut rng: ThreadRng = thread_rng();
        if spawn_stabilize {
            self.stabilize();
        }
        for _ in 0..spawn_count {
            let pos: Vec2 = {
                let mut pos = Vec2 {
                    x: rng.gen_range(spawn_radius..screen_width()-spawn_radius),
                    y: rng.gen_range(spawn_radius..screen_height()-spawn_radius),
                };
                'unsafe_pos: for _ in 0..spawn_safety_iterations {
                    for i in 0..self.verlet_objects.len() {
                        let obj = self.verlet_objects[i];
                        if !((obj.position_current - pos).len() < (obj.radius + spawn_radius) * spawn_safety_radius_factor) {
                            break 'unsafe_pos;
                        }
                    }
                    pos.x = rng.gen_range(spawn_radius..screen_width()-spawn_radius);
                    pos.y = rng.gen_range(spawn_radius..screen_height()-spawn_radius);
                }
                pos
            };
            self.spawn(pos, spawn_radius);
        }
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
        self.remove_oob_objs();
        self.solve_collisions();
        self.update_positions(dt);
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
        // create cell grid
        let grid_width: usize = (screen_width() / self.cell_size).ceil() as usize;
        let grid_height: usize = (screen_height() / self.cell_size).ceil() as usize;
        let mut grid: Vec<Vec<Vec<usize>>> = Vec::with_capacity(grid_height);
        for i in 0..grid_height {
            grid.push(Vec::with_capacity(grid_width));
            for _ in 0..grid_width {
                grid[i].push(vec![]);
            }
        }

        // assign to cells
        for i in 0..self.verlet_objects.len() {
            let Vec2{x, y} = self.verlet_objects[i].position_current;
            let grid_x: usize = (x / self.cell_size).floor() as usize;
            let grid_y: usize = (y / self.cell_size).floor() as usize;
            grid[grid_y][grid_x].push(i);
        }

        // resolve cells
        for y in 0..grid_height {
            for x in 0..grid_width {
                let current_cell: &Vec<usize> = &grid[y][x];
                // iterate over neighbours
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        let ox: isize = x as isize + dx;
                        let oy: isize = y as isize + dy;
                        if ox >= 0 && oy >= 0 && ox < grid_width as isize && oy < grid_height as isize {
                            let other_cell: &Vec<usize> = &grid[oy as usize][ox as usize];
                            self.solve_cell_collisions(current_cell, other_cell);
                        }
                    }
                }
            }
        }

        // old, naive collision resolution (~1600 objects before <60fps)
        // for i in 0..self.verlet_objects.len() {
        //     for k in i+1..self.verlet_objects.len() {
        //         let collision_axis: Vec2 = self.verlet_objects[i].position_current - self.verlet_objects[k].position_current;
        //         let dist: f32 = collision_axis.len();
        //         let radii: f32 = self.verlet_objects[i].radius + self.verlet_objects[k].radius;
        //         if dist < radii {
        //             let n: Vec2 = collision_axis / dist;
        //             let delta: f32 = radii - dist;
        //             self.verlet_objects[i].position_current += n * 0.5 * delta;
        //             self.verlet_objects[k].position_current -= n * 0.5 * delta;
        //         }
        //     }
        // }
    }

    pub fn solve_cell_collisions(&mut self, cell_1: &Vec<usize>, cell_2: &Vec<usize>) {
        for obj_index_1 in cell_1 {
            for obj_index_2 in cell_2 {
                if obj_index_1 != obj_index_2 {
                    self.solve_object_collision(*obj_index_1, *obj_index_2);
                }
            }
        }
    }

    pub fn solve_object_collision(&mut self, obj_index_1: usize, obj_index_2: usize) {
        let collision_axis: Vec2 = self.verlet_objects[obj_index_1].position_current - self.verlet_objects[obj_index_2].position_current;
        let dist: f32 = collision_axis.len();
        let radii: f32 = self.verlet_objects[obj_index_1].radius + self.verlet_objects[obj_index_2].radius;
        if dist < radii {
            let n: Vec2 = collision_axis / dist;
            let delta: f32 = radii - dist;
            self.verlet_objects[obj_index_1].position_current += n * 0.5 * delta;
            self.verlet_objects[obj_index_2].position_current -= n * 0.5 * delta;
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