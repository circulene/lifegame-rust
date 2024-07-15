use std::error;

use lifegame_core::{Cell, World};
use rand::Rng;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub nx: usize,
    pub ny: usize,
    pub world: World,
}

fn random_cells(nx: usize, ny: usize, alive_prob: f64) -> Vec<Cell> {
    let mut rng = rand::thread_rng();
    let size = nx * ny;
    (0..size)
        .map(|_| match rng.gen_bool(alive_prob) {
            true => Cell::Alive,
            false => Cell::Dead,
        })
        .collect::<Vec<_>>()
}

impl Default for App {
    fn default() -> Self {
        let (nx, ny) = (120, 60);
        let alive_prob = 0.2;
        let cells = random_cells(nx, ny, alive_prob);
        let world = World::new(nx, ny, cells);
        Self {
            running: true,
            nx,
            ny,
            world,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.world.next();
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
