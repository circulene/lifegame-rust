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
pub struct World {
    nx: usize,
    ny: usize,
    cells: BitVec,
    border: WorldBound,
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
            border: Plane,
        })
    }

    pub fn border(&mut self, border: WorldBound) {
        self.border = border;
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

    /// get cell with border consideration
    fn get_cell_with_border(&self, ix: isize, iy: isize) -> bool {
        let (nx, ny) = (self.nx as isize, self.ny as isize);
        match self.border {
            Plane => {
                if ix < 0 || iy < 0 || ix >= nx || iy >= ny {
                    CELL_DEAD
                } else {
                    self.cell(ix as usize, iy as usize)
                }
            }
            Torus => self.cell(
                get_index_with_cyclic_bound(ix, nx) as usize,
                get_index_with_cyclic_bound(iy, ny) as usize,
            ),
        }
    }

    /// get bitmap representing Moore neighbours in following order.
    /// [MSB] SE S SW E W NE N NW [LSB]
    fn neighbours(&self, ix: usize, iy: usize) -> BitVec {
        let (ix, iy) = (ix as isize, iy as isize);
        let bitmap: BitVec = to_bitvec(&[
            self.get_cell_with_border(ix - 1, iy - 1), // NW
            self.get_cell_with_border(ix, iy - 1),     // N
            self.get_cell_with_border(ix + 1, iy - 1), // NE
            self.get_cell_with_border(ix - 1, iy),     // W
            self.get_cell_with_border(ix + 1, iy),     // E
            self.get_cell_with_border(ix - 1, iy + 1), // SW
            self.get_cell_with_border(ix, iy + 1),     // S
            self.get_cell_with_border(ix + 1, iy + 1), // SE
        ]);
        bitmap
    }
}

fn to_bitvec(bits: &[bool]) -> BitVec {
    let mut bitvec = BitVec::with_capacity(bits.len());
    for bit in bits.iter() {
        bitvec.push(*bit);
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
            [[CELL_ALIVE, CELL_ALIVE], [CELL_ALIVE, CELL_ALIVE]].concat(),
        )?;
        assert_eq!(world.get_cell_with_border(-1, -1), CELL_DEAD);
        assert_eq!(world.get_cell_with_border(-1, 0), CELL_DEAD);
        assert_eq!(world.get_cell_with_border(0, 0), CELL_ALIVE);
        assert_eq!(world.get_cell_with_border(1, 1), CELL_ALIVE);
        assert_eq!(world.get_cell_with_border(2, 2), CELL_DEAD);

        // torus
        let mut world = World::new(
            2,
            2,
            [[CELL_ALIVE, CELL_DEAD], [CELL_DEAD, CELL_ALIVE]].concat(),
        )?;
        world.border(Torus);
        assert_eq!(world.get_cell_with_border(-1, -1), CELL_ALIVE);
        assert_eq!(world.get_cell_with_border(-1, 0), CELL_DEAD);
        assert_eq!(world.get_cell_with_border(0, 0), CELL_ALIVE);
        assert_eq!(world.get_cell_with_border(1, 1), CELL_ALIVE);
        assert_eq!(world.get_cell_with_border(2, 2), CELL_ALIVE);

        Ok(())
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
