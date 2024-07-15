use std::cmp::Ordering;

use crate::Cell::{Alive, Dead};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Cell {
    Dead,
    Alive,
}

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
                let fills = vec![Dead; num_fills];
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

    pub fn cell(&self, ix: usize, iy: usize) -> &Cell {
        &self.cells[iy * self.nx + ix]
    }

    pub fn next(&mut self) {
        let mut next_cells = Vec::with_capacity(self.cells.len());
        for iy in 0..self.ny {
            for ix in 0..self.nx {
                let cell = self.cell(ix, iy);
                let num_alive_neighbours = self
                    .neighbours(ix, iy)
                    .iter()
                    .filter(|cell| cell == &&&Alive)
                    .count();
                let next = match cell {
                    Dead => {
                        if num_alive_neighbours == 3 {
                            // born
                            Alive
                        } else {
                            Dead
                        }
                    }
                    Alive => {
                        if num_alive_neighbours <= 1 {
                            // underpopulated
                            Dead
                        } else if num_alive_neighbours == 2 || num_alive_neighbours == 3 {
                            // preserved
                            Alive
                        } else {
                            // overpopulated
                            Dead
                        }
                    }
                };
                next_cells.push(next);
            }
        }
        self.cells = next_cells;
    }

    fn neighbours(&self, ix: usize, iy: usize) -> Vec<&Cell> {
        let get_cell = |ix, iy| {
            if ix < 0 || iy < 0 || ix as usize >= self.nx || iy as usize >= self.ny {
                &Dead
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
    use super::Cell::{Alive, Dead};
    use super::*;

    #[test]
    fn test_world_new() {
        let space = World::new(2, 2, vec![Alive]);
        assert_eq!(space.cells(), &vec![Alive, Dead, Dead, Dead]);

        let space = World::new(2, 2, vec![Alive; 5]);
        assert_eq!(space.cells(), &vec![Alive; 4]);

        let space = World::new(2, 2, vec![Alive; 4]);
        assert_eq!(space.cells(), &vec![Alive; 4]);
    }

    #[test]
    fn rule_born() {
        let mut space = World::new(
            3,
            3,
            [
                [Alive, Alive, Dead],
                [Alive, Dead, Dead],
                [Dead, Dead, Dead],
            ]
            .concat(),
        );
        space.next();
        assert_eq!(
            space.cells(),
            &[
                [Alive, Alive, Dead],
                [Alive, Alive, Dead],
                [Dead, Dead, Dead]
            ]
            .concat()
        );
    }

    #[test]
    fn rule_preserved() {
        let mut space = World::new(
            4,
            4,
            [
                [Dead, Dead, Dead, Dead],
                [Dead, Alive, Alive, Dead],
                [Dead, Alive, Alive, Dead],
                [Dead, Dead, Dead, Dead],
            ]
            .concat(),
        );
        space.next();
        assert_eq!(
            space.cells(),
            &[
                [Dead, Dead, Dead, Dead],
                [Dead, Alive, Alive, Dead],
                [Dead, Alive, Alive, Dead],
                [Dead, Dead, Dead, Dead],
            ]
            .concat()
        );
    }

    #[test]
    fn rule_dead_with_underpopulated() {
        let mut space = World::new(
            3,
            3,
            [[Dead, Dead, Dead], [Dead, Alive, Alive], [Dead, Dead, Dead]].concat(),
        );
        space.next();
        assert_eq!(
            space.cells(),
            &[[Dead, Dead, Dead], [Dead, Dead, Dead], [Dead, Dead, Dead]].concat()
        );
    }

    #[test]
    fn rule_dead_with_overpopulated() {
        let mut space = World::new(
            3,
            3,
            [
                [Alive, Alive, Alive],
                [Alive, Alive, Dead],
                [Dead, Dead, Dead],
            ]
            .concat(),
        );
        space.next();
        assert_eq!(
            space.cells(),
            &[
                [Alive, Dead, Alive],
                [Alive, Dead, Alive],
                [Dead, Dead, Dead],
            ]
            .concat()
        );
    }
}
