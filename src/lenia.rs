use crate::life::*; 
use ggez::{graphics::Color, graphics::Canvas, Context, GameResult};
use rand::prelude::*;
use std::fmt;

/// Structure représentant une grille du jeu de la vie version Lénia.
#[derive(Clone)]
pub struct LeniaGrid {
    /// Vecteur contenant l'état actuel de chaque cellule de la grille.
    current_cells: Vec<u8>,
    /// Vecteur contenant l'état suivant de chaque cellule de la grille, utilisé pour éviter de copier la grille inutilement à chaque mise à jour.
    next_cells: Vec<u8>,
    /// Nombre de lignes de la grille.
    rows: usize,
    /// Nombre de colonnes de la grille.
    cols: usize,
    /// Indique si les bords de la grille sont connectés, formant une grille torique. Si vrai, les bords gauche et droit ainsi que les bords supérieur et inférieur sont connectés.
    toricgrid: bool, 
}

// Implémentation d'une méthode pour afficher la grille de Lenia
impl fmt::Display for LeniaGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        grid_fmt(f, self.rows, self.cols, &self.current_cells)
    }
}


impl Grid for LeniaGrid {
    fn display(&self) {
        println!("{}", self);
    }
    
    fn new(rows: usize, cols: usize, toricgrid: bool) -> Self {
        LeniaGrid {
            current_cells: vec![0; rows * cols],
            next_cells: vec![0; rows * cols],
            rows,
            cols,
            toricgrid: toricgrid,
        }
    }
    
    fn new_random(rows: usize, cols: usize, toricgrid: bool) -> Self {
        let mut rng: ThreadRng = rand::thread_rng();
        let mut current_cells: Vec<u8> = Vec::with_capacity(rows * cols);
        for _ in 0..(rows * cols) {
            let cell_state: u8 = rng.gen_range(0..2); // Génère aléatoirement 0 ou 1
            current_cells.push(cell_state);
        }

        LeniaGrid {
            current_cells,
            next_cells: vec![0; rows * cols],
            rows,
            cols,
            toricgrid: toricgrid,
        }
    }
    
    fn from_vect(cels: Vec<u8>, rows: usize, cols: usize, toricgrid: bool) -> Self  {
        // Vérifie si la longueur du vecteur correspond au nombre total de cellules dans la grille
        assert_eq!(cels.len(), rows * cols);

        LeniaGrid {
            current_cells: cels,
            next_cells: vec![0; rows * cols],
            rows,
            cols,
            toricgrid,
        }
    }
    
    fn set_color_alive(&mut self, color: Option<ggez::graphics::Color>)  {
        // These methods are not implemented and should not be used.
    }
    
    fn set_color_not_alive(&mut self, color: Option<ggez::graphics::Color>)  {
        // These methods are not implemented and should not be used.
    }
    
    /// Renvoie le nombre de lignes de la grille.
    fn rows(&self) -> usize {
        self.rows
    }

    /// Renvoie le nombre de colonnes de la grille.
    fn cols(&self) -> usize {
        self.cols
    }

    /// Indique si les bords de la grille sont connectés, formant une grille torique.
    fn is_toricgrid(&self) -> bool {
        self.toricgrid
    }

    fn current_cells(&self) -> &Vec<u8> {
        &self.current_cells
    }
    
    fn set_cell_state(&mut self, row: usize, col: usize, alive: u8) {
        grid_set_cell_state(row, col, alive, &mut self.current_cells, self.cols)
    }
    
    fn toggle_cell_state(&mut self, row: usize, col: usize) {
        // These methods are not implemented and should not be used.
    }
    
    fn is_alive(&self, row: usize, col: usize) -> bool {
        grid_is_alive(row, col, &self.current_cells, self.cols)
    }
    
    fn index(&self, row: usize, col: usize) -> usize {
        grid_index(row, col, self.cols)
    }
    
    fn count_neighbors(&self, row: usize, col: usize) -> usize {
        // finir
        grid_count_neighbors(
            row,
            col,
            &self.current_cells,
            self.rows,
            self.cols,
            self.toricgrid,
        )
    }
    
    fn update(&mut self) {
        // finir
        grid_update(
            &mut self.current_cells,
            &mut self.next_cells,
            self.rows,
            self.cols,
            self.toricgrid,
        );
    }
    
    fn draw(&self, ctx: &mut ggez::Context, canvas : &mut ggez::graphics::Canvas, cell_size: f32) -> ggez::GameResult {
        // finir
        draw_grid(ctx, canvas, self, cell_size,  Some(Color::from_rgb(0, 0, 255)), Some(Color::from_rgb(0, 0, 255)))
    }
}