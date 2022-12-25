use std::time::*;
use raylib::prelude::*;

pub mod grid;
use grid::Grid;

const INTERVAL: Duration = Duration::from_millis(800);

#[derive(PartialEq)]
enum State {
    PAUSED,
    RUN,
}

pub struct Game {
    last_time: Instant,
    grid: Grid,
    state: State,
}

impl Game {
    pub fn new(rl: &RaylibHandle, side_length: i32) -> Game {
        let screen_width = rl.get_screen_width();
        let screen_height = rl.get_screen_height();
        Game {
            last_time: Instant::now(),
            grid: Grid::new(
                            (screen_width / side_length) as usize,
                            (screen_height / side_length) as usize,
                            side_length as f32),
            state: State::PAUSED,
        }
    }

    pub fn toggle_run(&mut self) {
        match self.state {
            State::PAUSED => self.start(),
            State::RUN => self.pause(),
        }
    }

    pub fn start(&mut self) {
        self.state = State::RUN;
    }

    pub fn pause(&mut self) {
        self.state = State::PAUSED;
    }

    pub fn set_cell_alive_at_mouse_position(&mut self, pos: Vector2) {
        for cell in self.grid.cells.iter_mut() {
            if cell.rectangle().check_collision_point_rec(pos) {
                cell.set_alive();
                break;
            }
        }
    }

    pub fn apply_rule_logic(&mut self) { // can be made more readable
        if self.state == State::PAUSED || self.last_time.elapsed() < INTERVAL {
            return
        }

        let cell_will_die = self.cells_will_die();
        let cell_will_revive = self.cells_will_revive();

        for index in cell_will_die {
            self.grid.cells[index].set_dead();
        }

        for index in cell_will_revive {
            self.grid.cells[index].set_alive();
        }

        self.last_time = Instant::now();
    }

    fn cells_will_die(&self) -> Vec<usize> {
        (0..self.grid.cells.len())
        .filter(|i| {
            let living_count = self.grid.living_neighbor_count(*i);
            self.grid.cells[*i].is_alive() && (living_count != 2 && living_count != 3)
        })
        .collect()
    }

    fn cells_will_revive(&self) -> Vec<usize> {
        (0..self.grid.cells.len())
        .filter(|i| {
            let living_count = self.grid.living_neighbor_count(*i);
            !self.grid.cells[*i].is_alive() && living_count == 3
        })
        .collect()
    }

    pub fn draw_grid(&self, draw_handle: &mut RaylibDrawHandle) {
        for cell in self.grid.cells.iter() {
            draw_handle.draw_rectangle_rec(cell.rectangle(), cell.color());
            draw_handle.draw_rectangle_lines_ex(cell.rectangle(), 1, Color::WHITE);
        }
    }

    /* -------------- Debug -------------------- */
    pub fn debug(&self, rd: &mut RaylibDrawHandle) {
        for i in 0..self.grid.count {
            let neighbour_count = self.grid.living_neighbor_count(i);
            let count_string:&str = &(format!("{}", neighbour_count));
            let rec = self.grid.cells[i].rectangle();
            rd.draw_text(count_string, rec.x as i32, rec.y as i32, 50, Color::GREEN);
        }

        if self.state == State::PAUSED {
            rd.draw_text("Paused", 0, 0, 50, Color::RED);
        }

    }
}

