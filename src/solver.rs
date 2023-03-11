
use crate::{vector::Vec2, verlet::VerletObject};
use macroquad::prelude::{screen_width, screen_height};
use rand::{rngs::ThreadRng, thread_rng, Rng};

// this is to prevent "popcorn effect" and is less effective the more objects there are
// doubles as a default cell size (for smallest spawnable 1.0 radius objects)
pub const CELL_SIZE_RADIUS_FACTOR: f32 = 4.0;

pub struct Solver {
    pub verlet_objects: Vec<VerletObject>,
    pub cell_size: f32,
    pub cell_grid: Vec<Vec<Vec<usize>>>,

    pub gravity: Vec2,
    pub spawn_radius: f32,

    pub spawn_safety_radius_factor: f32,
    pub spawn_safety_iterations: usize,

    pub stabilize_on_spawn: bool,
    pub stabilize_on_oob: bool,

    pub min_object_count: usize,
    pub min_object_count_enforced: bool,
    pub max_object_count: usize,
    pub max_object_count_enforced: bool,

    pub apply_constraint_bottom: bool,
    pub apply_constraint_top: bool,
    pub apply_constraint_left: bool,
    pub apply_constraint_right: bool,
    pub apply_constraint_circle: bool,
}

impl Solver {
    pub fn new() -> Self {
        let grid_width: usize = (screen_width().ceil() / CELL_SIZE_RADIUS_FACTOR) as usize;
        let grid_height: usize = (screen_height().ceil() / CELL_SIZE_RADIUS_FACTOR) as usize;
        let mut grid: Vec<Vec<Vec<usize>>> = Vec::with_capacity(grid_height);
        for i in 0..grid_height {
            grid.push(Vec::with_capacity(grid_width));
            for _ in 0..grid_width {
                grid[i].push(vec![]);
            }
        }
        Self {
            verlet_objects: vec![],
            cell_size: CELL_SIZE_RADIUS_FACTOR,
            cell_grid: grid,

            gravity: Vec2 {
                x: 0.0,
                y: 1_000.0,
            },
            spawn_radius: 10.0,

            spawn_safety_radius_factor: 1.0,
            spawn_safety_iterations: 100,

            stabilize_on_spawn: false,
            stabilize_on_oob: false,

            min_object_count: 500,
            min_object_count_enforced: false,
            max_object_count: 5000,
            max_object_count_enforced: false,

            apply_constraint_bottom: true,
            apply_constraint_top: true,
            apply_constraint_left: true,
            apply_constraint_right: true,
            apply_constraint_circle: false,
        }
    }

    pub fn push(&mut self, obj: VerletObject) {
        // shrink back cell size
        // if self.verlet_objects.is_empty() {
        //     self.cell_size = CELL_SIZE_RADIUS_FACTOR;
        // }
        // optimize cell size (factor to prevent "popcorn effect")
        self.cell_size = self.cell_size.max(obj.radius * CELL_SIZE_RADIUS_FACTOR);
        self.verlet_objects.push(obj);
    }

    pub fn remove(&mut self, obj_index: usize) {
        self.verlet_objects.remove(obj_index);
    }

    pub fn remove_count(&mut self, obj_count: usize) {
        self.verlet_objects.drain(0..obj_count.min(self.verlet_objects.len()));
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

    pub fn spawn(&mut self, pos: Vec2) {
        self.push(VerletObject::new(pos, self.spawn_radius));
    }

    pub fn spawn_count(&mut self, spawn_count: usize) {
        let mut rng: ThreadRng = thread_rng();
        for _ in 0..spawn_count {
            let pos: Vec2 = {
                let mut pos = Vec2 {
                    x: rng.gen_range(self.spawn_radius..screen_width()-self.spawn_radius),
                    y: rng.gen_range(self.spawn_radius..screen_height()-self.spawn_radius),
                };
                'unsafe_pos: for _ in 0..self.spawn_safety_iterations {
                    for i in 0..self.verlet_objects.len() {
                        let obj = self.verlet_objects[i];
                        if !((obj.position_current - pos).len() < (obj.radius + self.spawn_radius) * self.spawn_safety_radius_factor) {
                            break 'unsafe_pos;
                        }
                    }
                    pos.x = rng.gen_range(self.spawn_radius..screen_width()-self.spawn_radius);
                    pos.y = rng.gen_range(self.spawn_radius..screen_height()-self.spawn_radius);
                }
                pos
            };
            self.spawn(pos);
        }
        if self.stabilize_on_spawn {
            self.stabilize();
        }
    }
    
    pub fn stabilize(&mut self) {
        for i in 0..self.verlet_objects.len() {
            self.verlet_objects[i].position_old = self.verlet_objects[i].position_current;
        }
    }

    pub fn accelerate_all(&mut self, intensity: f32, direction: f32) {
        let vec: Vec2 = Vec2 {
            x: direction.cos() * intensity,
            y: direction.sin() * intensity,
        };
        for i in 0..self.verlet_objects.len() {
            self.verlet_objects[i].accelerate(vec);
        }
    }

    pub fn update_with_substep(&mut self, dt: f32, substebs: usize) {
        let sub_dt: f32 = dt / substebs as f32;
        for _ in 0..substebs {
            self.update(sub_dt);
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.verlet_objects.is_empty() {self.enforce_object_count()};
        if self.verlet_objects.is_empty() {return};
        self.apply_gravity();
        self.apply_constraint();
        self.remove_oob_objs();
        self.solve_collisions();
        self.update_positions(dt);
        self.enforce_object_count();
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
        let mut max_radius: f32 = 1.0; // optimizing cell size for next update
        for i in (0..self.verlet_objects.len()).rev() {
            if self.apply_constraint_top && self.verlet_objects[i].position_current.y < self.verlet_objects[i].radius {
                self.verlet_objects[i].position_current.y = self.verlet_objects[i].radius;
            }
            if self.apply_constraint_bottom && self.verlet_objects[i].position_current.y > screen_height() - self.verlet_objects[i].radius {
                self.verlet_objects[i].position_current.y = screen_height() - self.verlet_objects[i].radius;
            }
            if self.apply_constraint_left && self.verlet_objects[i].position_current.x < self.verlet_objects[i].radius {
                self.verlet_objects[i].position_current.x = self.verlet_objects[i].radius;
            }
            if self.apply_constraint_right && self.verlet_objects[i].position_current.x > screen_width() - self.verlet_objects[i].radius {
                self.verlet_objects[i].position_current.x = screen_width() - self.verlet_objects[i].radius;
            }

            // if obj still outside contraints, handle oob
            if
                self.verlet_objects[i].position_current.is_nan() ||
                self.verlet_objects[i].position_current.y < -self.verlet_objects[i].radius ||
                self.verlet_objects[i].position_current.y > screen_height() + self.verlet_objects[i].radius ||
                self.verlet_objects[i].position_current.x < -self.verlet_objects[i].radius ||
                self.verlet_objects[i].position_current.x > screen_width() + self.verlet_objects[i].radius
            {
                self.verlet_objects.remove(i);
                if self.stabilize_on_oob {
                    self.stabilize();
                }
            } else {
                max_radius = max_radius.max(self.verlet_objects[i].radius);
            }
        }
        self.cell_size = max_radius * CELL_SIZE_RADIUS_FACTOR;
    }

    pub fn solve_collisions(&mut self) {
        // create cell grid
        let grid_width: usize = (screen_width() / self.cell_size).ceil() as usize;
        let grid_height: usize = (screen_height() / self.cell_size).ceil() as usize;

        if grid_height <= self.cell_grid.len() && grid_width <= self.cell_grid[0].len() {
            // only clear cells that are necessary
            for y in 0..grid_height {
                for x in 0..grid_width {
                    self.cell_grid[y][x].clear();
                }
            }
        } else {
            // up the size of the grid
            self.cell_grid = Vec::with_capacity(grid_height);
            for i in 0..grid_height {
                self.cell_grid.push(Vec::with_capacity(grid_width));
                for _ in 0..grid_width {
                    self.cell_grid[i].push(vec![]);
                }
            }
        }

        // assign to cells
        for i in 0..self.verlet_objects.len() {
            let Vec2{x, y} = self.verlet_objects[i].position_current;
            let grid_x: usize = (x / self.cell_size).floor() as usize;
            let grid_y: usize = (y / self.cell_size).floor() as usize;
            if (0..grid_width).contains(&grid_x) && (0..grid_height).contains(&grid_y) {
                self.cell_grid[grid_y][grid_x].push(i);
            }
        }

        // resolve cells
        for y in 0..grid_height {
            for x in 0..grid_width {
                //let current_cell: &Vec<usize> = &self.cell_grid[y][x];
                // iterate over neighbours
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        let ox: isize = x as isize + dx;
                        let oy: isize = y as isize + dy;
                        if ox >= 0 && oy >= 0 && ox < grid_width as isize && oy < grid_height as isize {
                            //let other_cell: &Vec<usize> = &self.cell_grid[oy as usize][ox as usize];
                            self.solve_cell_collisions((x, y), (ox as usize, oy as usize));
                        }
                    }
                }
            }
        }
    }

    // pub fn solve_cell_collisions(&mut self, cell_1: &Vec<usize>, cell_2: &Vec<usize>) {
    pub fn solve_cell_collisions(&mut self, cell_1_index: (usize, usize), cell_2_index: (usize, usize)) {
        let (x, y): (usize, usize) = cell_1_index;
        let (ox, oy): (usize, usize) = cell_2_index;
        for obj_index_1_index in 0..self.cell_grid[y][x].len() {
            for obj_index_2_index in 0..self.cell_grid[oy][ox].len() {
                let obj_index_1: usize = self.cell_grid[y][x][obj_index_1_index];
                let obj_index_2: usize = self.cell_grid[oy][ox][obj_index_2_index];
                if obj_index_1 != obj_index_2 {
                    self.solve_object_collision(obj_index_1, obj_index_2);
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
    }

    pub fn enforce_object_count(&mut self) {
        if self.min_object_count_enforced {
            let tmp_spawn_count: isize = self.min_object_count as isize - self.verlet_objects.len() as isize;
            if tmp_spawn_count > 0 {
                self.spawn_count(tmp_spawn_count as usize);
            }
        }
        if self.max_object_count_enforced {
            let tmp_spawn_count: isize = self.verlet_objects.len() as isize - self.max_object_count as isize;
            if tmp_spawn_count > 0 {
                self.remove_count(tmp_spawn_count as usize);
            }
        }
    }
}
