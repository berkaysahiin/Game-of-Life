mod cell;
use cell::Cell;

pub struct Grid {
    pub cells: Vec<Cell>,
    pub count_horz: usize,
    pub count_vert: usize,
    pub count: usize,
}

impl Grid {
    pub fn new(count_horz: usize, count_vert: usize, side_length: f32) -> Grid {
        let count = count_vert * count_horz;
        let mut cells = Vec::with_capacity(count);
        for i in 0..count{
            cells.push(Cell::new(
                    (i % count_horz) as f32 * side_length,
                    (i / count_horz) as f32 * side_length,
                    side_length));
        }

        Grid {
            cells, 
            count_horz,
            count_vert,
            count,
        }
    }

    pub fn living_neighbor_count(&self, index:usize) -> i32 {
        let neighbors = self.get_neighbors(index);

        neighbors.iter()
            .filter(|&i| self.cells[*i].is_alive())
            .count() as i32
    }

    fn get_neighbors(&self, index: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();
        let row = index / self.count_horz;
        let col = index % self.count_horz;

        // Check left neighbor
        if col > 0 {
            neighbors.push(index - 1);
        }

        // Check right neighbor
        if col < self.count_horz - 1 {
            neighbors.push(index + 1);
        }

        // Check top neighbor
        if row > 0 {
            neighbors.push(index - self.count_horz);
        }

        // Check bottom neighbor
        if row < self.count_vert - 1 {
            neighbors.push(index + self.count_horz);
        }

        // Check top-left diagonal neighbor
        if row > 0 && col > 0 {
            neighbors.push(index - self.count_horz - 1);
        }

        // Check top-right diagonal neighbor
        if row > 0 && col < self.count_horz - 1 {
            neighbors.push(index - self.count_horz + 1);
        }

        // Check bottom-left diagonal neighbor
        if row < self.count_vert - 1 && col > 0 {
            neighbors.push(index + self.count_horz - 1);
        }

        // Check bottom-right diagonal neighbor
        if row < self.count_vert - 1 && col < self.count_horz - 1 {
            neighbors.push(index + self.count_horz + 1);
        }

        neighbors
    }
}

