use std::fmt::Debug;

use anyhow::{Error, Result};

pub type Cell = u8;
pub const CELL_DEAD: Cell = 0;
pub const CELL_ALIVE: Cell = 1;

#[derive(Debug)]
pub struct World {
    nx: usize,
    ny: usize,
    cells: [Vec<Cell>; 2],
    present: usize,
    generation: usize,
}

impl World {
    /// Create a new world
    pub fn new(nx: usize, ny: usize, cells: &[Cell]) -> Result<World> {
        if cells.len() != nx * ny {
            return Err(Error::msg("invalid cell size."));
        }
        let nsize = nx * ny;
        Ok(World {
            nx,
            ny,
            cells: [
                process_boundary_cells(nx, ny, cells),
                vec![CELL_DEAD; nsize],
            ],
            present: 0,
            generation: 0,
        })
    }

    pub fn next(&mut self) {
        let next = (self.generation + 1) % 2;
        for iy in 1..(self.ny - 1) {
            for ix in 1..(self.nx - 1) {
                let present_cell = self.get_cell(self.present, ix, iy);
                let num_alive_neighbours = self.count_alive_neighbours(self.present, ix, iy);
                let next_cell = (num_alive_neighbours == 3
                    || (num_alive_neighbours == 2 && present_cell == CELL_ALIVE))
                    as u8;
                self.update_cell(next, ix, iy, next_cell);
            }
        }
        self.generation += 1;
        self.present = next;
    }

    #[inline]
    pub fn get_present_cell(&self, ix: usize, iy: usize) -> Cell {
        self.get_cell(self.present, ix, iy)
    }

    #[inline]
    fn get_cell(&self, index: usize, ix: usize, iy: usize) -> Cell {
        self.cells[index][self.nx * iy + ix]
    }

    #[inline]
    fn update_cell(&mut self, index: usize, ix: usize, iy: usize, cell: Cell) {
        self.cells[index][self.nx * iy + ix] = cell;
    }

    #[inline]
    fn count_alive_neighbours(&self, index: usize, ix: usize, iy: usize) -> u8 {
        self.get_cell(index, ix - 1, iy - 1) // NW
            + self.get_cell(index, ix, iy - 1)    // N
            + self.get_cell(index, ix + 1, iy - 1) // NE
            + self.get_cell(index, ix - 1, iy)    // W
            + self.get_cell(index, ix + 1, iy)    // E
            + self.get_cell(index, ix - 1, iy + 1) // SW
            + self.get_cell(index, ix, iy + 1)     // S
            + self.get_cell(index, ix + 1, iy + 1) // SE
    }
}

fn process_boundary_cells(nx: usize, ny: usize, cells: &[Cell]) -> Vec<Cell> {
    let mut processed_cells = vec![CELL_DEAD; nx * ny];
    for iy in 1..(ny - 1) {
        for ix in 1..(nx - 1) {
            processed_cells[nx * iy + ix] = cells[nx * iy + ix];
        }
    }
    processed_cells
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expand_boundary(nx: usize, ny: usize, cells: &[Cell]) -> Vec<Cell> {
        let mut result = vec![CELL_DEAD; (nx + 2) * (ny + 2)];
        for iy in 1..(ny + 1) {
            for ix in 1..(nx + 1) {
                result[(nx + 2) * iy + ix] = cells[nx * (iy - 1) + ix - 1];
            }
        }
        result
    }

    #[test]
    fn test_world_new() {
        let space = World::new(2, 2, &[CELL_ALIVE]);
        assert!(space.is_err());

        let space = World::new(2, 2, &[CELL_ALIVE; 5]);
        assert!(space.is_err());

        let space = World::new(
            4,
            4,
            &expand_boundary(2, 2, &[CELL_ALIVE, CELL_DEAD, CELL_DEAD, CELL_DEAD]),
        );
        assert!(space.is_ok());
        assert_eq!(
            &space.unwrap().cells[0],
            &expand_boundary(2, 2, &[CELL_ALIVE, CELL_DEAD, CELL_DEAD, CELL_DEAD])
        );
    }

    #[test]
    fn test_neighbour() -> Result<()> {
        // plane
        let world = World::new(
            4,
            4,
            &expand_boundary(
                2,
                2,
                &[[CELL_ALIVE, CELL_ALIVE], [CELL_ALIVE, CELL_ALIVE]].concat(),
            ),
        )?;
        assert_eq!(world.get_present_cell(0, 0), CELL_DEAD);
        assert_eq!(world.get_present_cell(0, 1), CELL_DEAD);
        assert_eq!(world.get_present_cell(1, 1), CELL_ALIVE);
        assert_eq!(world.get_present_cell(2, 2), CELL_ALIVE);
        assert_eq!(world.get_present_cell(3, 3), CELL_DEAD);

        Ok(())
    }

    #[test]
    fn rule_born() {
        let mut space = World::new(
            5,
            5,
            &expand_boundary(
                3,
                3,
                &[
                    [CELL_ALIVE, CELL_ALIVE, CELL_DEAD],
                    [CELL_ALIVE, CELL_DEAD, CELL_DEAD],
                    [CELL_DEAD, CELL_DEAD, CELL_DEAD],
                ]
                .concat(),
            ),
        )
        .unwrap();
        space.next();
        assert_eq!(
            &space.cells[space.present],
            &expand_boundary(
                3,
                3,
                &[
                    [CELL_ALIVE, CELL_ALIVE, CELL_DEAD],
                    [CELL_ALIVE, CELL_ALIVE, CELL_DEAD],
                    [CELL_DEAD, CELL_DEAD, CELL_DEAD]
                ]
                .concat()
            )
        );
    }

    #[test]
    fn rule_survive() {
        let mut space = World::new(
            6,
            6,
            &expand_boundary(
                4,
                4,
                &[
                    [CELL_DEAD, CELL_DEAD, CELL_DEAD, CELL_DEAD],
                    [CELL_DEAD, CELL_ALIVE, CELL_ALIVE, CELL_DEAD],
                    [CELL_DEAD, CELL_ALIVE, CELL_ALIVE, CELL_DEAD],
                    [CELL_DEAD, CELL_DEAD, CELL_DEAD, CELL_DEAD],
                ]
                .concat(),
            ),
        )
        .unwrap();
        space.next();
        assert_eq!(
            &space.cells[space.present],
            &expand_boundary(
                4,
                4,
                &[
                    [CELL_DEAD, CELL_DEAD, CELL_DEAD, CELL_DEAD],
                    [CELL_DEAD, CELL_ALIVE, CELL_ALIVE, CELL_DEAD],
                    [CELL_DEAD, CELL_ALIVE, CELL_ALIVE, CELL_DEAD],
                    [CELL_DEAD, CELL_DEAD, CELL_DEAD, CELL_DEAD],
                ]
                .concat()
            )
        );
    }

    #[test]
    fn rule_dead_with_underpopulated() {
        let mut space = World::new(
            5,
            5,
            &expand_boundary(
                3,
                3,
                &[
                    [CELL_DEAD, CELL_DEAD, CELL_DEAD],
                    [CELL_DEAD, CELL_ALIVE, CELL_ALIVE],
                    [CELL_DEAD, CELL_DEAD, CELL_DEAD],
                ]
                .concat(),
            ),
        )
        .unwrap();
        space.next();
        assert_eq!(
            &space.cells[space.present],
            &expand_boundary(
                3,
                3,
                &[
                    [CELL_DEAD, CELL_DEAD, CELL_DEAD],
                    [CELL_DEAD, CELL_DEAD, CELL_DEAD],
                    [CELL_DEAD, CELL_DEAD, CELL_DEAD]
                ]
                .concat()
            )
        );
    }

    #[test]
    fn rule_dead_with_overpopulated() {
        let mut space = World::new(
            5,
            5,
            &expand_boundary(
                3,
                3,
                &[
                    [CELL_ALIVE, CELL_ALIVE, CELL_ALIVE],
                    [CELL_ALIVE, CELL_ALIVE, CELL_DEAD],
                    [CELL_DEAD, CELL_DEAD, CELL_DEAD],
                ]
                .concat(),
            ),
        )
        .unwrap();
        space.next();
        assert_eq!(
            &space.cells[space.present],
            &expand_boundary(
                3,
                3,
                &[
                    [CELL_ALIVE, CELL_DEAD, CELL_ALIVE],
                    [CELL_ALIVE, CELL_DEAD, CELL_ALIVE],
                    [CELL_DEAD, CELL_DEAD, CELL_DEAD],
                ]
                .concat()
            )
        );
    }
}
