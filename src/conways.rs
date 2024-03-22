use crate::life::*;
use ggez::{Context, GameResult};
use rand::prelude::*;
use std::fmt;

/// Structure représentant une grille du jeu de la vie.
#[derive(Clone)]
pub struct ConwaysGrid {
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

// Implémentation d'une méthode pour afficher la grille
impl fmt::Display for ConwaysGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        grid_fmt(f, self.rows, self.cols, &self.current_cells)
    }
}

impl Grid for ConwaysGrid {
    fn display(&self) {
        println!("{}", self);
    }
    /// Initialise une nouvelle grille du jeu de la vie avec des cellules mortes.
    ///
    /// # Arguments
    ///
    /// * `rows` - Le nombre de lignes de la grille.
    /// * `cols` - Le nombre de colonnes de la grille.
    /// * `toricgrid` - Indique si les bords de la grille sont connectés, formant une grille torique.
    ///
    /// # Exemple
    ///
    /// ```
    /// use crate::lifers::ConwaysGrid;
    /// use crate::lifers::Grid;
    ///
    /// // Crée une nouvelle grille 3x3 avec des cellules mortes
    /// let grid = ConwaysGrid::new(3, 3, true);
    /// ```
    fn new(rows: usize, cols: usize, toricgrid: bool) -> ConwaysGrid {
        ConwaysGrid {
            current_cells: vec![0; rows * cols],
            next_cells: vec![0; rows * cols],
            rows,
            cols,
            toricgrid: toricgrid,
        }
    }

    /// Initialise une nouvelle grille du jeu de la vie avec des cellules dont l'état est aléatoire.
    ///
    /// # Arguments
    ///
    /// * `rows` - Le nombre de lignes de la grille.
    /// * `cols` - Le nombre de colonnes de la grille.
    /// * `toricgrid` - Le nombre de colonnes de la grille.
    ///
    /// # Exemple
    ///
    /// ```
    /// use crate::lifers::ConwaysGrid;
    /// use crate::lifers::Grid;
    ///
    /// // Crée une nouvelle grille 3x3 avec des cellules dont l'état est aléatoire
    /// let grid = ConwaysGrid::new_random(3, 3, true);
    /// ```
    fn new_random(rows: usize, cols: usize, toricgrid: bool) -> ConwaysGrid {
        let mut rng: ThreadRng = rand::thread_rng();
        let mut current_cells: Vec<u8> = Vec::with_capacity(rows * cols);
        for _ in 0..(rows * cols) {
            let cell_state: u8 = rng.gen_range(0..2); // Génère aléatoirement 0 ou 1
            current_cells.push(cell_state);
        }

        ConwaysGrid {
            current_cells,
            next_cells: vec![0; rows * cols],
            rows,
            cols,
            toricgrid: toricgrid,
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

    /// Définit l'état d'une cellule spécifiée dans la grille.
    ///
    /// # Arguments
    ///
    /// * `row` - L'indice de la ligne de la cellule dans la grille.
    /// * `col` - L'indice de la colonne de la cellule dans la grille.
    /// * `alive` - Un booléen indiquant si la cellule doit être mise à l'état vivant (`true`) ou mort (`false`).
    ///
    /// # Exemple
    ///
    /// ```
    /// use crate::lifers::ConwaysGrid;
    /// use crate::lifers::Grid;
    ///
    /// let mut grid = ConwaysGrid::new(3, 3, true);
    ///
    /// // Met une cellule à l'état vivant
    /// grid.set_cell_state(1, 1, 1);
    ///
    /// // Vérifie si la cellule est maintenant vivante
    /// assert_eq!(grid.is_alive(1, 1), true);
    ///
    /// // Met une cellule à l'état mort
    /// grid.set_cell_state(1, 1, 0);
    ///
    /// // Vérifie si la cellule est maintenant morte
    /// assert_eq!(grid.is_alive(1, 1), false);
    /// ```
    fn set_cell_state(&mut self, row: usize, col: usize, alive: u8) {
        grid_set_cell_state(row, col, alive, &mut self.current_cells, self.cols)
    }

    /// Inverse l'état d'une cellule spécifiée dans la grille.
    ///
    /// Si la cellule est vivante, elle devient morte, et si elle est morte, elle devient vivante.
    ///
    /// # Arguments
    ///
    /// * `row` - L'indice de la ligne de la cellule dans la grille.
    /// * `col` - L'indice de la colonne de la cellule dans la grille.
    ///
    /// # Exemple
    ///
    /// ```
    /// use crate::lifers::ConwaysGrid;
    /// use crate::lifers::Grid;
    ///
    /// let mut grid = ConwaysGrid::new(3, 3, true);
    ///
    /// // Inverse l'état de la cellule
    /// grid.toggle_cell_state(1, 1);
    ///
    /// // Vérifie si la cellule est maintenant vivante
    /// assert_eq!(grid.is_alive(1, 1), true);
    /// ```
    fn toggle_cell_state(&mut self, row: usize, col: usize) {
        grid_toggle_cell_state(row, col, &mut self.current_cells, self.cols)
    }

    /// Vérifie si une cellule spécifiée dans la grille est vivante.
    ///
    /// # Arguments
    ///
    /// * `row` - L'indice de la ligne de la cellule dans la grille.
    /// * `col` - L'indice de la colonne de la cellule dans la grille.
    ///
    /// # Returns
    ///
    /// `true` si la cellule spécifiée est vivante, sinon `false`.
    ///
    /// # Exemple
    ///
    /// ```
    /// use crate::lifers::ConwaysGrid;
    /// use crate::lifers::Grid;
    ///
    /// let mut grid = ConwaysGrid::new(3, 3, true);
    ///
    /// // Vérifie si la cellule en haut à gauche est vivante
    /// assert_eq!(grid.is_alive(0, 0), false);
    /// ```
    fn is_alive(&self, row: usize, col: usize) -> bool {
        grid_is_alive(row, col, &self.current_cells, self.cols)
    }

    /// Calcule l'index d'une cellule dans le vecteur représentant la grille.
    ///
    /// # Arguments
    ///
    /// * `row` - L'indice de la ligne de la cellule dans la grille.
    /// * `col` - L'indice de la colonne de la cellule dans la grille.
    ///
    /// # Returns
    ///
    /// L'index de la cellule dans le vecteur représentant la grille.
    ///
    /// # Exemple
    ///
    /// ```
    /// use crate::lifers::ConwaysGrid;
    /// use crate::lifers::Grid;
    ///
    /// let grid = ConwaysGrid::new(3, 3, true);
    ///
    /// // Calcule l'index de la cellule en bas à droite dans une grille 3x3
    /// let index = grid.index(2, 2);
    /// assert_eq!(index, 8);
    /// ```
    fn index(&self, row: usize, col: usize) -> usize {
        grid_index(row, col, self.cols)
    }

    /// Compte le nombre de voisins vivants d'une cellule spécifiée dans la grille.
    ///
    /// # Arguments
    ///
    /// * `row` - L'indice de la ligne de la cellule dans la grille.
    /// * `col` - L'indice de la colonne de la cellule dans la grille.
    ///
    /// # Returns
    ///
    /// Le nombre de voisins vivants de la cellule spécifiée.
    ///
    /// # Exemple
    ///
    /// ```
    /// use crate::lifers::ConwaysGrid;
    /// use crate::lifers::Grid;
    ///
    /// let mut grid = ConwaysGrid::new(3, 3, true);
    ///
    /// grid.update(); // Met à jour l'état de la grille au step suivant
    ///
    /// // Compte les voisins vivants d'une cellule
    /// assert_eq!(grid.count_neighbors(1, 1), 0);
    ///
    /// // Met une cellule à l'état vivant
    /// grid.set_cell_state(1, 0, 1);
    ///
    /// // Compte les voisins vivants d'une cellule
    /// assert_eq!(grid.count_neighbors(1, 1), 1);
    ///
    /// ```
    fn count_neighbors(&self, row: usize, col: usize) -> usize {
        grid_count_neighbors(
            row,
            col,
            &self.current_cells,
            self.rows,
            self.cols,
            self.toricgrid,
        )
    }

    /// Met à jour l'état de la grille selon les règles du jeu de la vie.
    ///
    /// Cette méthode parcourt chaque cellule de la grille, compte ses voisins vivants et applique les règles du jeu pour mettre à jour son état.
    ///
    /// # Exemple
    ///
    /// ```
    /// use crate::lifers::ConwaysGrid;
    /// use crate::lifers::Grid;
    ///
    /// let mut grid = ConwaysGrid::new(3, 3, true);
    ///
    /// // Met à jour l'état de la grille selon les règles du jeu de la vie
    /// grid.update();
    /// ```
    fn update(&mut self) {
        grid_update(
            &mut self.current_cells,
            &mut self.next_cells,
            self.rows,
            self.cols,
            self.toricgrid,
        );
    }
    /// Dessine la grille en utilisant le contexte `ctx` spécifié et la taille de cellule `cell_size`.
    ///
    /// Cette méthode appelle la fonction `draw_grid` pour dessiner la grille en utilisant le contexte
    /// `ctx` et la taille de cellule `cell_size`.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Le contexte du jeu.
    /// * `cell_size` - La taille de chaque cellule de la grille.
    ///
    /// # Erreurs
    ///
    /// Cette méthode peut retourner une erreur de type `GameError` si une erreur survient lors du dessin.
    ///
    /// # Exemple
    ///
    /// ```
    /// // grid = ConwaysGrid::new_random(5, 5, true);
    /// // grid.draw(&mut ctx, cell_size)?;
    ///
    /// ```
    ///
    /// Cette méthode peut être utilisée pour dessiner une grille de jeu dans une fenêtre `ggez`.
    fn draw(&self, ctx: &mut Context, cell_size: f32) -> GameResult {
        draw_grid(ctx, self, cell_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_neighbors_toricgrid_enabled() {
        // Création d'une grille 3x3 avec toricgrid activé
        let mut grid: ConwaysGrid = ConwaysGrid::new(3, 3, true);
        grid.set_cell_state(0, 1, 1);
        grid.set_cell_state(2, 1, 1);

        // Vérification des voisins de la cellule row 1 colonne 1
        assert_eq!(grid.count_neighbors(1, 1), 2);
        // Vérification des voisins de la cellule en haut à gauche
        assert_eq!(grid.count_neighbors(0, 0), 2);
        // Vérification des voisins de la cellule en bas à droite
        assert_eq!(grid.count_neighbors(2, 2), 2);
    }

    #[test]
    fn test_count_neighbors_toricgrid_disabled() {
        // Création d'une grille 3x3 avec toricgrid désactivé
        let mut grid: ConwaysGrid = ConwaysGrid::new(3, 3, false);
        grid.set_cell_state(0, 1, 1);
        grid.set_cell_state(2, 1, 1);

        // Vérification des voisins de la cellule row 1 colonne 1
        assert_eq!(grid.count_neighbors(1, 1), 2);
        // Vérification des voisins de la cellule en haut à gauche
        assert_eq!(grid.count_neighbors(0, 0), 1);
        // Vérification des voisins de la cellule en bas à droite
        assert_eq!(grid.count_neighbors(2, 2), 1);
    }
}
