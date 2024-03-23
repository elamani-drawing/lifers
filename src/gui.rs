use ggez::input::keyboard::KeyInput;
use ggez::{Context, input::mouse::MouseButton, input::keyboard::KeyCode, GameResult};  
use ggez::event::EventHandler;
use ggez::timer;

use crate::Grid;

pub struct LifeGui<G> {
    grid: G,
    cell_size: f32, 
    is_paused: bool,
    fps: u32
}

impl<G: Grid> LifeGui<G> {
    /// Crée une nouvelle instance de `LifeGui` avec la référence à la grille spécifiée.
    pub fn new(grid: G, cell_size: f32) -> Self { 
        LifeGui { grid, cell_size , is_paused:false, fps: 60}
    }
    /// Méthode pour modifier le FPS
    pub fn set_fps(&mut self, fps: u32) {
        self.fps = fps;
    }
    
}

impl<G: Grid> EventHandler for LifeGui<G> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Régule la mise à jour en fonction des FPS cibles (par exemple, 60 FPS)
        while ctx.time.check_update_time(self.fps) {
            // Vérifie si le jeu est en pause avant de mettre à jour la grille
            if !self.is_paused {
                self.grid.update();
            }
        }
        timer::yield_now(); // Facultatif : permet de libérer le CPU pour d'autres tâches
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult { 
        self.grid.draw(ctx, self.cell_size)
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult { 
        if button == MouseButton::Left { 
            // Convertir les coordonnées de la souris en indices de cellule
            let row = (y / self.cell_size) as usize;
            let col = (x / self.cell_size) as usize;

            // Vérifier que les indices sont valides
            if row < self.grid.rows() && col < self.grid.cols() {
                // Inverser l'état de la cellule (de morte à vivante ou de vivante à morte)
                self.grid.toggle_cell_state(row, col);
            }
        }
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult  {
        // Inverse l'état de la pause lorsque la touche "Espace" est enfoncée 
        if input.keycode == Some(KeyCode::Space) { 
            self.is_paused = !self.is_paused;
        }
        Ok(())
    }

    
}