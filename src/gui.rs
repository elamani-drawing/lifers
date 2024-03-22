use ggez::{Context, GameResult};  
use ggez::event::EventHandler;

use crate::Grid;

pub struct LifeGui<G> {
    grid: G,
    cell_size: f32, 
}

impl<G: Grid> LifeGui<G> {
    /// Crée une nouvelle instance de `LifeGui` avec la référence à la grille spécifiée.
    pub fn new(grid: G, cell_size: f32) -> Self { 
        LifeGui { grid, cell_size }
    }
}

impl<G: Grid> EventHandler for LifeGui<G> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.grid.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult { 
        self.grid.draw(ctx, self.cell_size)
    }
}