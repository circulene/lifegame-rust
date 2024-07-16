use anyhow::{Error, Result};
use bit_vec::BitVec;

pub type Cell = bool;
pub const CELL_DEAD: bool = false;
pub const CELL_ALIVE: bool = true;

#[derive(Debug)]
pub struct World {
    nx: usize,
    ny: usize,
    cells: BitVec,
}

impl World {
    /// Create a new world
    pub fn new(nx: usize, ny: usize, cells: Vec<Cell>) -> Result<World> {
        if cells.len() != nx * ny {
            return Err(Error::msg("invalid cell size."));
        }
        Ok(World {
            nx,
            ny,
            cells: to_bitvec(&cells),
        })
    }

    fn bit_index(&self, ix: usize, iy: usize) -> usize {
        iy * self.nx + ix
    }

    pub fn cell(&self, ix: usize, iy: usize) -> Cell {
        self.cells[self.bit_index(ix, iy)]
    }

    pub fn next(&mut self) {
        let mut next_cells = BitVec::from_elem(self.cells.len(), false);
        for iy in 0..self.ny {
            for ix in 0..self.nx {
                let cell = self.cell(ix, iy);
                let num_alive_neighbours = self.neighbours(ix, iy).iter().filter(|x| *x).count();
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
                next_cells.set(self.bit_index(ix, iy), next);
            }
        }
        self.cells = next_cells;
    }

    fn neighbours(&self, ix: usize, iy: usize) -> BitVec {
        let get_cell = |ix, iy| {
            if ix < 0 || iy < 0 || ix as usize >= self.nx || iy as usize >= self.ny {
                CELL_DEAD
            } else {
                self.cell(ix as usize, iy as usize)
            }
        };
        let (ix, iy) = (ix as isize, iy as isize);
        let bitmap: BitVec = to_bitvec(&[
            get_cell(ix - 1, iy - 1),
            get_cell(ix - 1, iy),
            get_cell(ix - 1, iy + 1),
            get_cell(ix, iy - 1),
            get_cell(ix, iy + 1),
            get_cell(ix + 1, iy - 1),
            get_cell(ix + 1, iy),
            get_cell(ix + 1, iy + 1),
        ]);
        bitmap
    }
}

fn to_bitvec(bits: &[bool]) -> BitVec {
    let mut bitvec = BitVec::from_elem(bits.len(), false);
    for (i, bit) in bits.iter().enumerate() {
        bitvec.set(i, *bit);
    }
    bitvec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_new() {
        let space = World::new(2, 2, vec![CELL_ALIVE]);
        assert!(space.is_err());

        let space = World::new(2, 2, vec![CELL_ALIVE; 5]);
        assert!(space.is_err());

        let space = World::new(2, 2, vec![CELL_ALIVE, CELL_DEAD, CELL_DEAD, CELL_DEAD]);
        assert!(space.is_ok());
        assert_eq!(
            &space.unwrap().cells,
            &to_bitvec(&[CELL_ALIVE, CELL_DEAD, CELL_DEAD, CELL_DEAD])
        );
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
        )
        .unwrap();
        space.next();
        assert_eq!(
            &space.cells,
            &to_bitvec(
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
            [
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
            &space.cells,
            &to_bitvec(
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
            [
                [CELL_DEAD, CELL_DEAD, CELL_DEAD],
                [CELL_DEAD, CELL_ALIVE, CELL_ALIVE],
                [CELL_DEAD, CELL_DEAD, CELL_DEAD],
            ]
            .concat(),
        )
        .unwrap();
        space.next();
        assert_eq!(
            &space.cells,
            &to_bitvec(
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
            [
                [CELL_ALIVE, CELL_ALIVE, CELL_ALIVE],
                [CELL_ALIVE, CELL_ALIVE, CELL_DEAD],
                [CELL_DEAD, CELL_DEAD, CELL_DEAD],
            ]
            .concat(),
        )
        .unwrap();
        space.next();
        assert_eq!(
            &space.cells,
            &to_bitvec(
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
