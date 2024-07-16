use std::cmp::Ordering;

pub type Cell = bool;
pub const CELL_DEAD: bool = false;
pub const CELL_ALIVE: bool = true;

#[derive(Debug)]
pub struct World {
    nx: usize,
    ny: usize,
    cells: Vec<Cell>,
}

impl World {
    /// Create a new world
    pub fn new(nx: usize, ny: usize, cells: Vec<Cell>) -> World {
        let size = nx * ny;
        let resized_cells = match cells.len().cmp(&size) {
            Ordering::Less => {
                let num_fills = size - cells.len();
                let fills = vec![CELL_DEAD; num_fills];
                [cells, fills].concat()
            }
            Ordering::Greater => cells[0..size].to_vec(),
            Ordering::Equal => cells,
        };
        World {
            nx,
            ny,
            cells: resized_cells,
        }
    }

    pub fn cell(&self, ix: usize, iy: usize) -> Cell {
        self.cells[iy * self.nx + ix]
    }

    pub fn next(&mut self) {
        let mut next_cells = Vec::with_capacity(self.cells.len());
        for iy in 0..self.ny {
            for ix in 0..self.nx {
                let cell = self.cell(ix, iy);
                let num_alive_neighbours = self
                    .neighbours(ix, iy)
                    .iter()
                    .filter(|cell| cell == &&CELL_ALIVE)
                    .count();
                let next = match cell {
                    CELL_DEAD => {
                        if num_alive_neighbours == 3 {
                            // born
                            CELL_ALIVE
                        } else {
                            CELL_DEAD
                        }
                    }
                    CELL_ALIVE => {
                        if num_alive_neighbours <= 1 {
                            // underpopulated
                            CELL_DEAD
                        } else if num_alive_neighbours == 2 || num_alive_neighbours == 3 {
                            // survive
                            CELL_ALIVE
                        } else {
                            // overpopulated
                            CELL_DEAD
                        }
                    }
                };
                next_cells.push(next);
            }
        }
        self.cells = next_cells;
    }

    fn neighbours(&self, ix: usize, iy: usize) -> Vec<Cell> {
        let get_cell = |ix, iy| {
            if ix < 0 || iy < 0 || ix as usize >= self.nx || iy as usize >= self.ny {
                CELL_DEAD
            } else {
                self.cell(ix as usize, iy as usize)
            }
        };
        let (ix, iy) = (ix as isize, iy as isize);
        vec![
            get_cell(ix - 1, iy - 1),
            get_cell(ix - 1, iy),
            get_cell(ix - 1, iy + 1),
            get_cell(ix, iy - 1),
            get_cell(ix, iy + 1),
            get_cell(ix + 1, iy - 1),
            get_cell(ix + 1, iy),
            get_cell(ix + 1, iy + 1),
        ]
    }

    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_new() {
        let space = World::new(2, 2, vec![CELL_ALIVE]);
        assert_eq!(
            space.cells(),
            &vec![CELL_ALIVE, CELL_DEAD, CELL_DEAD, CELL_DEAD]
        );

        let space = World::new(2, 2, vec![CELL_ALIVE; 5]);
        assert_eq!(space.cells(), &vec![CELL_ALIVE; 4]);

        let space = World::new(2, 2, vec![CELL_ALIVE; 4]);
        assert_eq!(space.cells(), &vec![CELL_ALIVE; 4]);
    }

    #[test]
    fn rule_born() {
        let mut space = World::new(
            3,
            3,
            [
                [CELL_ALIVE, CELL_ALIVE, CELL_DEAD],
                [CELL_ALIVE, CELL_DEAD, CELL_DEAD],
                [CELL_DEAD, CELL_DEAD, CELL_DEAD],
            ]
            .concat(),
        );
        space.next();
        assert_eq!(
            space.cells(),
            &[
                [CELL_ALIVE, CELL_ALIVE, CELL_DEAD],
                [CELL_ALIVE, CELL_ALIVE, CELL_DEAD],
                [CELL_DEAD, CELL_DEAD, CELL_DEAD]
            ]
            .concat()
        );
    }

    #[test]
    fn rule_survive() {
        let mut space = World::new(
            4,
            4,
            [
                [CELL_DEAD, CELL_DEAD, CELL_DEAD, CELL_DEAD],
                [CELL_DEAD, CELL_ALIVE, CELL_ALIVE, CELL_DEAD],
                [CELL_DEAD, CELL_ALIVE, CELL_ALIVE, CELL_DEAD],
                [CELL_DEAD, CELL_DEAD, CELL_DEAD, CELL_DEAD],
            ]
            .concat(),
        );
        space.next();
        assert_eq!(
            space.cells(),
            &[
                [CELL_DEAD, CELL_DEAD, CELL_DEAD, CELL_DEAD],
                [CELL_DEAD, CELL_ALIVE, CELL_ALIVE, CELL_DEAD],
                [CELL_DEAD, CELL_ALIVE, CELL_ALIVE, CELL_DEAD],
                [CELL_DEAD, CELL_DEAD, CELL_DEAD, CELL_DEAD],
            ]
            .concat()
        );
    }

    #[test]
    fn rule_dead_with_underpopulated() {
        let mut space = World::new(
            3,
            3,
            [
                [CELL_DEAD, CELL_DEAD, CELL_DEAD],
                [CELL_DEAD, CELL_ALIVE, CELL_ALIVE],
                [CELL_DEAD, CELL_DEAD, CELL_DEAD],
            ]
            .concat(),
        );
        space.next();
        assert_eq!(
            space.cells(),
            &[
                [CELL_DEAD, CELL_DEAD, CELL_DEAD],
                [CELL_DEAD, CELL_DEAD, CELL_DEAD],
                [CELL_DEAD, CELL_DEAD, CELL_DEAD]
            ]
            .concat()
        );
    }

    #[test]
    fn rule_dead_with_overpopulated() {
        let mut space = World::new(
            3,
            3,
            [
                [CELL_ALIVE, CELL_ALIVE, CELL_ALIVE],
                [CELL_ALIVE, CELL_ALIVE, CELL_DEAD],
                [CELL_DEAD, CELL_DEAD, CELL_DEAD],
            ]
            .concat(),
        );
        space.next();
        assert_eq!(
            space.cells(),
            &[
                [CELL_ALIVE, CELL_DEAD, CELL_ALIVE],
                [CELL_ALIVE, CELL_DEAD, CELL_ALIVE],
                [CELL_DEAD, CELL_DEAD, CELL_DEAD],
            ]
            .concat()
        );
    }
}
