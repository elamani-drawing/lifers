use std::fmt;
use rand::prelude::*;

/// Structure représentant une grille du jeu de la vie.
pub struct Grid {
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
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let index = self.index(row, col);
                let symbol = if self.current_cells[index] >= 1 { "■" } else { "□" };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
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
    /// use crate::lifers::Grid;
    ///
    /// // Crée une nouvelle grille 3x3 avec des cellules mortes
    /// let grid = Grid::new(3, 3, true);
    /// ```
    pub fn new(rows: usize, cols: usize, toricgrid : bool) -> Grid {
        Grid {
            current_cells: vec![0; rows * cols],
            next_cells: vec![0; rows * cols],
            rows,
            cols,
            toricgrid : toricgrid,
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
    /// use crate::lifers::Grid;
    ///
    /// // Crée une nouvelle grille 3x3 avec des cellules dont l'état est aléatoire
    /// let grid = Grid::new_random(3, 3, true);
    /// ```
    pub fn new_random(rows: usize, cols: usize, toricgrid : bool) -> Grid {
        let mut rng : ThreadRng = rand::thread_rng();
        let mut current_cells : Vec<u8> = Vec::with_capacity(rows * cols);
        for _ in 0..(rows * cols) {
            let cell_state : u8 = rng.gen_range(0..2); // Génère aléatoirement 0 ou 1
            current_cells.push(cell_state);
        }

        Grid {
            current_cells,
            next_cells: vec![0; rows * cols],
            rows,
            cols,
            toricgrid: toricgrid,
        }
    }

    /// Renvoie le nombre de lignes de la grille.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Renvoie le nombre de colonnes de la grille.
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Indique si les bords de la grille sont connectés, formant une grille torique.
    pub fn is_toricgrid(&self) -> bool {
        self.toricgrid
    }

    pub fn current_cells(&self) -> &Vec<u8> {
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
    /// use crate::lifers::Grid;
    ///
    /// let mut grid = Grid::new(3, 3, true);
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
    pub fn set_cell_state(&mut self, row: usize, col: usize, alive: u8) {
        let index : usize = self.index(row, col);
        println!("--al {}", alive);
        self.current_cells[index] = alive; 
        println!("{:?}", self.current_cells);
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
    /// use crate::lifers::Grid;
    ///
    /// let mut grid = Grid::new(3, 3, true);
    ///
    /// // Inverse l'état de la cellule
    /// grid.toggle_cell_state(1, 1);
    ///
    /// // Vérifie si la cellule est maintenant vivante
    /// assert_eq!(grid.is_alive(1, 1), true);
    /// ```
    pub fn toggle_cell_state(&mut self, row: usize, col: usize) {
        let index = self.index(row, col);
        // Inverse l'état de la cellule : de vivante à morte ou de morte à vivante
        self.current_cells[index] = if self.current_cells[index] >= 1 { 0 } else { 1 };
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
    /// use crate::lifers::Grid;
    ///
    /// let mut grid = Grid::new(3, 3, true); 
    ///
    /// // Vérifie si la cellule en haut à gauche est vivante
    /// assert_eq!(grid.is_alive(0, 0), false);
    /// ```
    pub fn is_alive(&self, row: usize, col: usize) -> bool{
        self.current_cells[self.index(row, col)] >= 1
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
    /// use crate::lifers::Grid;
    ///
    /// let grid = Grid::new(3, 3, true);
    ///
    /// // Calcule l'index de la cellule en bas à droite dans une grille 3x3
    /// let index = grid.index(2, 2);
    /// assert_eq!(index, 8);
    /// ```
    pub fn index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
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
    /// use crate::lifers::Grid;
    ///
    /// let mut grid = Grid::new(3, 3, true);
    ///
    /// grid.update(); // Met à jour l'état de la grille au step suivant
    ///
    /// // Compte les voisins vivants de la cellule centrale
    /// assert_eq!(grid.count_neighbors(1, 1), 0); 
    ///
    /// // Met une cellule à l'état vivant
    /// grid.set_cell_state(1, 0, 1);
    ///
    /// // Compte les voisins vivants de la cellule centrale
    /// assert_eq!(grid.count_neighbors(1, 1), 1); 
    ///
    /// ```
    pub fn count_neighbors(&self, row: usize, col: usize) -> usize {
        let mut count = 0;
        // Parcours des cellules voisines de la cellule spécifiée
        for i in (row as isize - 1)..=(row as isize + 1) {
            for j in (col as isize - 1)..=(col as isize + 1) {
                if self.toricgrid {
                    // Si la grille est torique, ajuste les indices des bords
                    let mut i_wrapped = i;
                    let mut j_wrapped = j;
                    // Ajustement des indices pour les bords
                    if i_wrapped < 0 {
                        i_wrapped = self.rows as isize - 1;
                    } else if i_wrapped >= self.rows as isize {
                        i_wrapped = 0;
                    }
                    if j_wrapped < 0 {
                        j_wrapped = self.cols as isize - 1;
                    } else if j_wrapped >= self.cols as isize {
                        j_wrapped = 0;
                    }
                    let index = self.index(i_wrapped as usize, j_wrapped as usize);
                    // Vérification et comptage des voisins vivants
                    if !(i == row as isize && j == col as isize) && self.current_cells[index] >= 1 {
                        count += 1;
                    }
                }  else {
                    // Si la grille n'est pas torique, on compte les voisins normalement
                    if i >= 0
                        && i < self.rows as isize
                        && j >= 0
                        && j < self.cols as isize
                        && !(i == row as isize && j == col as isize)
                    {
                        let index = self.index(i as usize, j as usize);
                        if self.current_cells[index] >= 1 {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    /// Met à jour l'état de la grille selon les règles du jeu de la vie.
    ///
    /// Cette méthode parcourt chaque cellule de la grille, compte ses voisins vivants et applique les règles du jeu pour mettre à jour son état.
    ///
    /// # Exemple
    ///
    /// ```
    /// use crate::lifers::Grid;
    ///
    /// let mut grid = Grid::new(3, 3, true);
    ///
    /// // Met à jour l'état de la grille selon les règles du jeu de la vie
    /// grid.update();
    /// ```
    pub fn update(&mut self) {
        // Parcours de chaque cellule de la grille
        for row in 0..self.rows {
            for col in 0..self.cols {
                let index = self.index(row, col); // Calcul de l'index de la cellule actuelle
                let neighbors_count = self.count_neighbors(row, col); // Comptage des voisins vivants de la cellule actuelle
                // Application des règles du jeu de la vie pour mettre à jour l'état de la cellule
                self.next_cells[index] = match (self.current_cells[index], neighbors_count) {
                    // Si la cellule est vivante et a 2 ou 3 voisins vivants, elle reste vivante
                    (cell, 2) | (cell, 3) if cell > 1 => cell,
                    // Si la cellule est morte et a exactement 3 voisins vivants, elle devient vivante
                    (_, 3) => 1,
                    // Sinon, la cellule meurt
                    _ => 0,
                };
            }
        }
        // Échange des vecteurs d'état actuel avec le prochain pour mettre à jour l'état de la grille
        std::mem::swap(&mut self.current_cells, &mut self.next_cells);
    }

}
 
 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_count_neighbors_toricgrid_enabled() {
        // Création d'une grille 3x3 avec toricgrid activé 
        let mut grid : Grid = Grid::new(3, 3, true);
        grid.set_cell_state(0, 1, 1);
        grid.set_cell_state(2, 1, 1);

        // Vérification des voisins de la cellule centrale
        assert_eq!(grid.count_neighbors(1, 1), 2);
        // Vérification des voisins de la cellule en haut à gauche
        assert_eq!(grid.count_neighbors(0, 0), 2);
        // Vérification des voisins de la cellule en bas à droite
        assert_eq!(grid.count_neighbors(2, 2), 2);
    }

    #[test]
    pub fn test_count_neighbors_toricgrid_disabled() {
        // Création d'une grille 3x3 avec toricgrid désactivé 
        let mut grid : Grid = Grid::new(3, 3, false);
        grid.set_cell_state(0, 1, 1);
        grid.set_cell_state(2, 1, 1);

        // Vérification des voisins de la cellule centrale
        assert_eq!(grid.count_neighbors(1, 1), 2);
        // Vérification des voisins de la cellule en haut à gauche
        assert_eq!(grid.count_neighbors(0, 0), 1);
        // Vérification des voisins de la cellule en bas à droite
        assert_eq!(grid.count_neighbors(2, 2), 1);
    }
}