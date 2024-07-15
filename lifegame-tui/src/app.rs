use std::error;

use lifegame_core::{Cell, World};
use rand::Rng;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Eq, PartialEq)]
pub enum AppState {
    Run,
    Pause,
    Quit,
}

/// Application.
#[derive(Debug)]
pub struct App {
    pub gen: u64,
    pub state: AppState,
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
            gen: 0,
            state: AppState::Pause,
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
        if self.state == AppState::Run {
            self.gen = self.gen.saturating_add(1);
            self.world.next();
        }
    }

    pub fn toggle(&mut self) {
        match self.state {
            AppState::Pause => self.state = AppState::Run,
            AppState::Run => self.state = AppState::Pause,
            _ => (),
        };
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.state = AppState::Quit;
    }
}
