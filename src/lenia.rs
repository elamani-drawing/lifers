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
    /// Filtre à appliqué l'ors de la convolution par filtre pour le calcul de la valeur du voisinage d'une cellule.
    filter: Vec<u8>,
    /// Le nombre de voisins dans le filtre.
    neighbors_filter : usize, 
    /// La fonction de croissance.
    growth_function : fn(u8)-> u8,
    /// Une fonction permettant de convertir la valeur de la cellule (0 à 255) en couleur.
    cell_value_color_mappeur : fn(u8) -> Color,
}

// Implémentation d'une méthode pour afficher la grille de Lenia
impl fmt::Display for LeniaGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        grid_fmt(f, self.rows, self.cols, &self.current_cells)
    }
}

impl LeniaGrid {
    /// Calcule la valeur voisinage résultante de l'application d'une convolution par filtre sur les valeurs des cellules voisines
    /// autour d'une cellule spécifiée dans la grille, en utilisant le filtre défini (par défaut c'est le filtrage en anneau).
    ///
    /// # Arguments
    ///
    /// * `row` - L'indice de la ligne de la cellule dans la grille.
    /// * `col` - L'indice de la colonne de la cellule dans la grille.
    ///
    /// # Returns
    ///
    /// La valeur résultante de l'application de la convolution sur les valeurs des cellules voisines. 
    pub fn get_value_neighbors_lenia(&self, row: usize, col: usize) -> u16 {
        grid_convolution_neighbors_lenia(
            row,
            col,
            &self.current_cells,
            self.rows,
            self.cols,
            self.toricgrid,
            &self.filter
        ) 
    }
    
    /// Setter pour editer le vecteur contenant les valeurs du filtre à appliquer lors du calcul de la valeur du voisinage
    pub fn set_filter(&mut self, filter : Vec<u8>) {
        let mut count_neighbors : usize = 0;
        for &value in &filter {
            if value == 1 {
                count_neighbors += 1;
            }
        }
        // Il doit y avoir au moin un voisin dans le filtre
        assert!(count_neighbors > 0);
        self.neighbors_filter = count_neighbors;
        self.filter = filter;
    }

    /// Setter pour editer la function de croissance.
    pub fn set_growth_function(&mut self, growth_function : fn(u8) -> u8) {
        self.growth_function = growth_function;
    }

    /// Setter pour editer la function qui indique la couleur de la cellue par rapport à sa valeur.
    pub fn set_cell_value_color_mappeur(&mut self, cell_value_color_mappeur : fn(u8)-> Color) {
        self.cell_value_color_mappeur = cell_value_color_mappeur;
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
            filter :  vec![
                1, 1, 1,
                1, 0, 1,
                1, 1, 1,
            ], // Par defaut nous utilisant un filtre en anneau
            neighbors_filter : 8,
            growth_function : lenia_growth_function_default,
            cell_value_color_mappeur : map_cell_value_to_color,
        }
    }
    
    fn new_random(rows: usize, cols: usize, toricgrid: bool) -> Self {
        let mut rng: ThreadRng = rand::thread_rng();
        let mut current_cells: Vec<u8> = Vec::with_capacity(rows * cols);
        for _ in 0..(rows * cols) {
            let cell_state: u8 = rng.gen_range(0..255); 
            current_cells.push(cell_state);
        }

        LeniaGrid {
            current_cells,
            next_cells: vec![0; rows * cols],
            rows,
            cols,
            toricgrid: toricgrid,
            filter :  vec![
                1, 1, 1,
                1, 0, 1,
                1, 1, 1,
            ], // Par defaut nous utilisant un filtre en anneau
            neighbors_filter : 8,
            growth_function : lenia_growth_function_default,
            cell_value_color_mappeur : map_cell_value_to_color,
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
            filter :  vec![
                1, 1, 1,
                1, 0, 1,
                1, 1, 1,
            ], // Par defaut nous utilisant un filtre en anneau
            neighbors_filter : 8,
            growth_function : lenia_growth_function_default,
            cell_value_color_mappeur : map_cell_value_to_color,
        }
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
    
    
    fn is_alive(&self, row: usize, col: usize) -> bool {
        grid_is_alive(row, col, &self.current_cells, self.cols)
    }
    
    fn index(&self, row: usize, col: usize) -> usize {
        grid_index(row, col, self.cols)
    }
     
    /// Compte le nombre de voisins vivants d'une cellule spécifiée dans la grille.
    ///
    /// # Arguments
    ///
    /// * `row` - L'indice de la ligne de la cellule dans la grille.
    /// * `col` - L'indice de la colonne de la cellule dans la grille.
    /// * `current_cells` - Vecteur contenant l'état actuel de chaque cellule de la grille.
    /// * `rows` - Nombre de lignes de la grille.
    /// * `cols` - Nombre de colonnes de la grille.
    /// * `toricgrid` - Indique si les bords de la grille sont connectés, formant une grille torique.
    ///
    /// # Returns
    ///
    /// Le nombre de voisins vivants de la cellule spécifiée.
    ///
    fn count_neighbors(&self, row: usize, col: usize) -> usize { 
        grid_count_neighbors_conways(
            row,
            col,
            &self.current_cells,
            self.rows,
            self.cols,
            self.toricgrid,
        )
    }
    
    fn update(&mut self) { 
        grid_update_lenia(
            &mut self.current_cells,
            &mut self.next_cells,
            self.rows,
            self.cols,
            self.toricgrid,
            &self.filter,
            self.neighbors_filter, 
            self.growth_function
        );
    }
    
    fn draw(&self, ctx: &mut Context, canvas : &mut Canvas, cell_size: f32) -> GameResult { 
        draw_grid_lenia(ctx, canvas, self, cell_size, self.cell_value_color_mappeur)
    }
}