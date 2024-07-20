use std::fmt::Debug;

use anyhow::{Error, Result};
use bit_vec::BitVec;
use WorldBound::{Plane, Torus};

pub type Cell = bool;
pub const CELL_DEAD: bool = false;
pub const CELL_ALIVE: bool = true;

#[derive(Debug, PartialEq, Eq)]
pub enum WorldBound {
    Plane,
    Torus,
}

#[inline]
fn get_index_with_cyclic_bound(ix: isize, bound: isize) -> isize {
    if ix < 0 {
        ((ix + bound) % bound).abs()
    } else if ix >= bound {
        ix % bound
    } else {
        ix
    }
}

#[derive(Debug)]
struct WorldConfig {
    nx: usize,
    ny: usize,
    cells: BitVec,
}

trait CellLocatable: Debug {
    #[inline]
    fn get_cell_index(&self, config: &WorldConfig, ix: usize, iy: usize) -> usize {
        iy * config.nx + ix
    }

    /// get cell at exact position
    #[inline]
    fn get_cell(&self, config: &WorldConfig, ix: usize, iy: usize) -> Cell {
        config.cells[self.get_cell_index(config, ix, iy)]
    }

    /// get neighbour cell. cell index (ix, iy) could be negative or larger than world.
    fn get_neighbour_cell(&self, config: &WorldConfig, ix: isize, iy: isize) -> Cell;
}

#[derive(Debug)]
struct PlaneBoundedCellLocator {}

impl CellLocatable for PlaneBoundedCellLocator {
    fn get_neighbour_cell(&self, config: &WorldConfig, ix: isize, iy: isize) -> Cell {
        if ix < 0 || iy < 0 || ix >= config.nx as isize || iy >= config.ny as isize {
            CELL_DEAD
        } else {
            self.get_cell(config, ix as usize, iy as usize)
        }
    }
}

#[derive(Debug)]
struct TorusBoundedCellLocator {}

impl CellLocatable for TorusBoundedCellLocator {
    fn get_neighbour_cell(&self, config: &WorldConfig, ix: isize, iy: isize) -> Cell {
        self.get_cell(
            config,
            get_index_with_cyclic_bound(ix, config.nx as isize) as usize,
            get_index_with_cyclic_bound(iy, config.ny as isize) as usize,
        )
    }
}

fn make_cell_identifier(wb: WorldBound) -> Box<dyn CellLocatable> {
    match wb {
        Torus => Box::new(TorusBoundedCellLocator {}),
        Plane => Box::new(PlaneBoundedCellLocator {}),
    }
}

#[derive(Debug)]
pub struct World {
    config: WorldConfig,
    locator: Box<dyn CellLocatable>,
}

impl World {
    /// Create a new world
    pub fn new(nx: usize, ny: usize, cells: &[Cell]) -> Result<World> {
        if cells.len() != nx * ny {
            return Err(Error::msg("invalid cell size."));
        }
        Ok(World {
            config: WorldConfig {
                nx: nx + 2,
                ny: ny + 2,
                cells: to_bitvec(nx, ny, cells),
            },
            locator: make_cell_identifier(Plane),
        })
    }

    pub fn set_bound(&mut self, wb: WorldBound) {
        self.locator = make_cell_identifier(wb);
    }

    pub fn next(&mut self) {
        let mut next_cells = BitVec::from_elem(self.config.cells.len(), false);
        for iy in 1..self.config.ny + 1 {
            for ix in 1..self.config.nx + 1 {
                let cell = self.get_cell(ix, iy);
                let num_alive_neighbours = self.count_alive_neighbours(ix, iy);
                let next = num_alive_neighbours == 3 || (num_alive_neighbours == 2 && cell);
                next_cells.set(self.get_cell_index(ix, iy), next);
            }
        }
        self.config.cells = next_cells;
    }

    #[inline]
    fn get_cell_index(&self, ix: usize, iy: usize) -> usize {
        ix + iy
    }

    #[inline]
    pub fn get_cell(&self, ix: usize, iy: usize) -> Cell {
        self.locator.get_cell(&self.config, ix, iy)
    }

    #[inline]
    fn count_alive_neighbours(&self, ix: usize, iy: usize) -> u8 {
        self.get_cell(ix - 1, iy - 1) as u8 // NW
            + self.get_cell(ix, iy - 1) as u8     // N
            + self.get_cell(ix + 1, iy - 1) as u8 // NE
            + self.get_cell(ix - 1, iy) as u8    // W
            + self.get_cell(ix + 1, iy) as u8    // E
            + self.get_cell(ix - 1, iy + 1) as u8 // SW
            + self.get_cell(ix, iy + 1) as u8     // S
            + self.get_cell(ix + 1, iy + 1) as u8 // SE
    }
}

fn to_bitvec(nx: usize, ny: usize, bits: &[bool]) -> BitVec {
    let mut bitvec = BitVec::from_elem((nx + 2) * (ny + 2), CELL_DEAD);
    for iy in 1..(ny + 1) {
        for ix in 1..(nx + 1) {
            bitvec.set(nx * iy + ix, bits[(nx - 1) * (iy - 1) + ix - 1]);
        }
    }
    bitvec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_new() {
        let space = World::new(2, 2, &[CELL_ALIVE]);
        assert!(space.is_err());

        let space = World::new(2, 2, &[CELL_ALIVE; 5]);
        assert!(space.is_err());

        let space = World::new(2, 2, &[CELL_ALIVE, CELL_DEAD, CELL_DEAD, CELL_DEAD]);
        assert!(space.is_ok());
        assert_eq!(
            &space.unwrap().config.cells,
            &to_bitvec(2, 2, &[CELL_ALIVE, CELL_DEAD, CELL_DEAD, CELL_DEAD])
        );
    }

    #[test]
    fn test_cyclic_bound() {
        assert_eq!(get_index_with_cyclic_bound(-21, 10), 1);
        assert_eq!(get_index_with_cyclic_bound(-1, 10), 9);
        assert_eq!(get_index_with_cyclic_bound(0, 10), 0);
        assert_eq!(get_index_with_cyclic_bound(9, 10), 9);
        assert_eq!(get_index_with_cyclic_bound(10, 10), 0);
        assert_eq!(get_index_with_cyclic_bound(21, 10), 1);
    }

    #[test]
    fn test_neighbour() -> Result<()> {
        // plane
        let world = World::new(
            2,
            2,
            &[[CELL_ALIVE, CELL_ALIVE], [CELL_ALIVE, CELL_ALIVE]].concat(),
        )?;
        assert_eq!(world.get_cell(-1, -1), CELL_DEAD);
        assert_eq!(world.get_cell(-1, 0), CELL_DEAD);
        assert_eq!(world.get_cell(0, 0), CELL_ALIVE);
        assert_eq!(world.get_cell(1, 1), CELL_ALIVE);
        assert_eq!(world.get_cell(2, 2), CELL_DEAD);

        // torus
        let mut world = World::new(
            2,
            2,
            &[[CELL_ALIVE, CELL_DEAD], [CELL_DEAD, CELL_ALIVE]].concat(),
        )?;
        world.set_bound(Torus);
        assert_eq!(world.get_neighbour_cell(-1, -1), CELL_ALIVE);
        assert_eq!(world.get_neighbour_cell(-1, 0), CELL_DEAD);
        assert_eq!(world.get_neighbour_cell(0, 0), CELL_ALIVE);
        assert_eq!(world.get_neighbour_cell(1, 1), CELL_ALIVE);
        assert_eq!(world.get_neighbour_cell(2, 2), CELL_ALIVE);

        Ok(())
    }

    #[test]
    fn rule_born() {
        let mut space = World::new(
            3,
            3,
            &[
                [CELL_ALIVE, CELL_ALIVE, CELL_DEAD],
                [CELL_ALIVE, CELL_DEAD, CELL_DEAD],
                [CELL_DEAD, CELL_DEAD, CELL_DEAD],
            ]
            .concat(),
        )
        .unwrap();
        space.next();
        assert_eq!(
            &space.config.cells,
            &to_bitvec(
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
            4,
            4,
            &[
                [CELL_DEAD, CELL_DEAD, CELL_DEAD, CELL_DEAD],
                [CELL_DEAD, CELL_ALIVE, CELL_ALIVE, CELL_DEAD],
                [CELL_DEAD, CELL_ALIVE, CELL_ALIVE, CELL_DEAD],
                [CELL_DEAD, CELL_DEAD, CELL_DEAD, CELL_DEAD],
            ]
            .concat(),
        )
        .unwrap();
        space.next();
        assert_eq!(
            &space.config.cells,
            &to_bitvec(
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
            3,
            3,
            &[
                [CELL_DEAD, CELL_DEAD, CELL_DEAD],
                [CELL_DEAD, CELL_ALIVE, CELL_ALIVE],
                [CELL_DEAD, CELL_DEAD, CELL_DEAD],
            ]
            .concat(),
        )
        .unwrap();
        space.next();
        assert_eq!(
            &space.config.cells,
            &to_bitvec(
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
            3,
            3,
            &[
                [CELL_ALIVE, CELL_ALIVE, CELL_ALIVE],
                [CELL_ALIVE, CELL_ALIVE, CELL_DEAD],
                [CELL_DEAD, CELL_DEAD, CELL_DEAD],
            ]
            .concat(),
        )
        .unwrap();
        space.next();
        assert_eq!(
            &space.config.cells,
            &to_bitvec(
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
