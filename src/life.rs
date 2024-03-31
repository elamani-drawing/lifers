use std::fmt;
use std::f64::consts::PI;
use ggez::{graphics::{self, Canvas, Color, Mesh, Rect}, Context, GameResult};

pub trait Grid {
    // Méthode pour afficher la grille
    fn display(&self);

    /// Initialise une nouvelle grille du jeu de la vie avec des cellules mortes.
    ///
    /// # Arguments
    ///
    /// * `rows` - Le nombre de lignes de la grille.
    /// * `cols` - Le nombre de colonnes de la grille.
    /// * `toricgrid` - Indique si les bords de la grille sont connectés, formant une grille torique.
    ///
    fn new(rows: usize, cols: usize, toricgrid: bool) -> Self;

    /// Initialise une nouvelle grille du jeu de la vie avec des cellules dont l'état est aléatoire.
    ///
    /// # Arguments
    ///
    /// * `rows` - Le nombre de lignes de la grille.
    /// * `cols` - Le nombre de colonnes de la grille.
    /// * `toricgrid` - Le nombre de colonnes de la grille.
    ///
    fn new_random(rows: usize, cols: usize, toricgrid: bool) -> Self;
    
    /// Méthode pour créer une grille à partir d'un vecteur de cellules
    fn from_vect(cels: Vec<u8>, rows: usize, cols: usize, toricgrid: bool) -> Self ;
 
    /// Méthode pour renvoyer le nombre de lignes de la grille
    fn rows(&self) -> usize;

    /// Méthode pour renvoyer le nombre de colonnes de la grille
    fn cols(&self) -> usize;

    /// Méthode pour indiquer si les bords de la grille sont connectés (forme une grille torique)
    fn is_toricgrid(&self) -> bool;

    /// Méthode pour renvoyer une référence vers le vecteur contenant l'état actuel des cellules
    fn current_cells(&self) -> &Vec<u8>;

    /// Définit l'état d'une cellule spécifiée dans la grille.
    ///
    /// # Arguments
    ///
    /// * `row` - L'indice de la ligne de la cellule dans la grille.
    /// * `col` - L'indice de la colonne de la cellule dans la grille.
    /// * `alive` - Un booléen indiquant si la cellule doit être mise à l'état vivant (`true`) ou mort (`false`).
    ///
    fn set_cell_state(&mut self, row: usize, col: usize, alive: u8);

    /// Vérifie si une cellule spécifiée dans la grille est vivante.
    ///
    /// # Arguments
    ///
    /// * `row` - L'indice de la ligne de la cellule dans la grille.
    /// * `col` - L'indice de la colonne de la cellule dans la grille.
    ///
    fn is_alive(&self, row: usize, col: usize) -> bool;

    /// Calcule l'index d'une cellule dans le vecteur représentant la grille.
    ///
    /// # Arguments
    ///
    /// * `row` - L'indice de la ligne de la cellule dans la grille.
    /// * `col` - L'indice de la colonne de la cellule dans la grille.
    ///
    fn index(&self, row: usize, col: usize) -> usize;

    /// Compte le nombre de voisins vivants d'une cellule spécifiée dans la grille.
    ///
    /// # Arguments
    ///
    /// * `row` - L'indice de la ligne de la cellule dans la grille.
    /// * `col` - L'indice de la colonne de la cellule dans la grille.
    ///
    fn count_neighbors(&self, row: usize, col: usize) -> usize;

    /// Met à jour l'état de la grille selon les règles du jeu de la vie.
    ///
    /// Cette méthode parcourt chaque cellule de la grille, compte ses voisins vivants et applique les règles du jeu pour mettre à jour son état.
    fn update(&mut self);

    /// Dessine la grille en utilisant le contexte `ctx` spécifié et la taille de cellule `cell_size`.
    ///
    /// Cette méthode appelle la fonction `draw_grid` pour dessiner la grille en utilisant le contexte
    /// `ctx` et la taille de cellule `cell_size`.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Le contexte du jeu.
    /// * `canvas` - Le canva sur lequel dessiner.
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
    /// // grid.draw(&mut ctx, &mut canvas, cell_size)?;
    ///
    /// ```
    ///
    /// Cette méthode peut être utilisée pour dessiner une grille de jeu dans une fenêtre `ggez`.
    fn draw(&self, ctx: &mut Context, canvas : &mut Canvas, cell_size: f32) -> GameResult;
}

/// Calcule l'index d'une cellule dans le vecteur représentant la grille.
///
/// # Arguments
///
/// * `row` - L'indice de la ligne de la cellule dans la grille.
/// * `col` - L'indice de la colonne de la cellule dans la grille.
/// * `cols` - Le nombre de colonnes de la grille.
///
/// # Returns
///
/// L'index de la cellule dans le vecteur représentant la grille.
///
/// # Exemple
///
/// ```rust
/// use crate::lifers::grid_index;
///
/// let rows = 3;
/// let cols = 3;
///
/// // Calcule l'index de la cellule en bas à droite dans une grille 3x3
/// let index = grid_index(2, 2, cols);
/// assert_eq!(index, 8);
/// ```
pub fn grid_index(row: usize, col: usize, cols: usize) -> usize {
    row * cols + col
}

/// Affiche la représentation textuelle de la grille dans le formatter spécifié.
///
/// # Arguments
///
/// * `f` - Un mutable référence vers un `fmt::Formatter` pour écrire la représentation textuelle.
/// * `rows` - Le nombre de lignes de la grille.
/// * `cols` - Le nombre de colonnes de la grille.
/// * `current_cells` - Vecteur contenant l'état actuel de chaque cellule de la grille.
///
/// # Exemple
///
/// ```rust
/// use std::fmt;
/// use crate::lifers::grid_fmt;
///
/// pub struct ConwaysGrid {
///     current_cells: Vec<u8>,  
///     rows: usize,
///     cols: usize,
/// }
///
/// impl fmt::Display for ConwaysGrid {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         grid_fmt(f, self.rows, self.cols, &self.current_cells)
///     }
/// }
/// ```
pub fn grid_fmt(
    f: &mut fmt::Formatter<'_>,
    rows: usize,
    cols: usize,
    current_cells: &Vec<u8>,
) -> fmt::Result {
    for row in 0..rows {
        for col in 0..cols {
            let index = grid_index(row, col, cols);
            let symbol = if current_cells[index] >= 1 {
                "■"
            } else {
                "□"
            };
            write!(f, "{}", symbol)?;
        }
        writeln!(f)?;
    }
    Ok(())
}

/// Définit l'état d'une cellule spécifiée dans la grille.
///
/// # Arguments
///
/// * `row` - L'indice de la ligne de la cellule dans la grille.
/// * `col` - L'indice de la colonne de la cellule dans la grille.
/// * `alive` - Un entier indiquant l'état de la cellule : 0 pour morte, 1 pour vivante.
/// * `current_cells` - Vecteur contenant l'état actuel de chaque cellule de la grille.
/// * `cols` - Nombre de colonnes de la grille.
///
/// # Example
///
/// ```
/// use crate::lifers::grid_set_cell_state;
///
/// let mut current_cells = vec![
///     0, 0, 0,
///     1, 0, 1,
///     0, 0, 0,
/// ];
/// let cols = 3;
///
/// // Définit l'état de la cellule en haut à gauche à vivante
/// grid_set_cell_state(0, 0, 1, &mut current_cells, cols);
/// // Vérifie si la cellule en haut à gauche est maintenant vivante
/// assert_eq!(current_cells[0], 1);
/// // Définit l'état de la cellule en bas à droite à morte
/// grid_set_cell_state(2, 2, 0, &mut current_cells, cols);
/// // Vérifie si la cellule en bas à droite est maintenant morte
/// assert_eq!(current_cells[8], 0);
/// ```
pub fn grid_set_cell_state(
    row: usize,
    col: usize,
    alive: u8,
    current_cells: &mut Vec<u8>,
    cols: usize,
) {
    let index: usize = grid_index(row, col, cols);
    current_cells[index] = alive;
}

/// Inverse l'état d'une cellule spécifiée dans la grille.
///
/// Si la cellule est vivante, elle devient morte, et si elle est morte, elle devient vivante.
///
/// # Arguments
///
/// * `row` - L'indice de la ligne de la cellule dans la grille.
/// * `col` - L'indice de la colonne de la cellule dans la grille.
/// * `current_cells` - Vecteur contenant l'état actuel de chaque cellule de la grille.
/// * `cols` - Nombre de colonnes de la grille.
///
/// # Example
///
/// ```
/// use crate::lifers::grid_toggle_cell_state;
///
/// let mut current_cells = vec![
///     0, 0, 0,
///     1, 0, 1,
///     0, 0, 0,
/// ];
/// let cols = 3;
///
/// // Inverse l'état de la cellule en haut à gauche
/// grid_toggle_cell_state(0, 0, &mut current_cells, cols);
/// // Vérifie si la cellule en haut à gauche est maintenant vivante
/// assert_eq!(current_cells[0], 1);
/// // Inverse à nouveau l'état de la même cellule
/// grid_toggle_cell_state(0, 0, &mut current_cells, cols);
/// // Vérifie si la cellule en haut à gauche est maintenant morte
/// assert_eq!(current_cells[0], 0);
/// ```
pub fn grid_toggle_cell_state(row: usize, col: usize, current_cells: &mut Vec<u8>, cols: usize) {
    let index: usize = grid_index(row, col, cols);
    // Inverse l'état de la cellule : de vivante à morte ou de morte à vivante
    current_cells[index] = if current_cells[index] >= 1 { 0 } else { 1 };
}

/// Vérifie si une cellule spécifiée dans la grille est vivante.
///
/// # Arguments
///
/// * `row` - L'indice de la ligne de la cellule dans la grille.
/// * `col` - L'indice de la colonne de la cellule dans la grille.
/// * `current_cells` - Vecteur contenant l'état actuel de chaque cellule de la grille.
/// * `cols` - Nombre de colonnes de la grille.
///
/// # Returns
///
/// `true` si la cellule spécifiée est vivante, sinon `false`.
///
/// # Example
///
/// ```
/// use crate::lifers::grid_is_alive;
///
/// let current_cells = vec![
///     0, 0, 0,
///     1, 0, 1,
///     0, 0, 0,
/// ];
/// let cols = 3;
///
/// // Vérifie si la cellule en haut à droite est vivante
/// assert_eq!(grid_is_alive(0, 2, &current_cells, cols), false);
/// // Vérifie si la cellule en row 1 et colonne 0 est vivante
/// assert_eq!(grid_is_alive(1, 0, &current_cells, cols), true);
/// // Vérifie si la cellule en bas à droite est vivante
/// assert_eq!(grid_is_alive(2, 2, &current_cells, cols), false);
/// ```
pub fn grid_is_alive(row: usize, col: usize, current_cells: &Vec<u8>, cols: usize) -> bool {
    current_cells[grid_index(row, col, cols)] >= 1
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
/// # Exemple
///
/// ```
/// use crate::lifers::grid_count_neighbors_conways;
///
/// let current_cells = vec![
///     0, 0, 0,
///     2, 0, 2,
///     0, 0, 0,
/// ];
/// let rows = 3;
/// let cols = 3;
/// let toricgrid = true;
///
/// // Compte les voisins vivants de la cellule centrale
/// let neighbors_count = grid_count_neighbors_conways(1, 1, &current_cells, rows, cols, toricgrid);
/// ```
pub fn grid_count_neighbors_conways(
    row: usize,
    col: usize,
    current_cells: &[u8],
    rows: usize,
    cols: usize,
    toricgrid: bool,
) -> usize {
    let mut count = 0;
    // Parcours des cellules voisines de la cellule spécifiée
    for i in (row as isize - 1)..=(row as isize + 1) {
        for j in (col as isize - 1)..=(col as isize + 1) {
            if toricgrid {
                // Si la grille est torique, ajuste les indices des bords
                let mut i_wrapped = i;
                let mut j_wrapped = j;
                // Ajustement des indices pour les bords
                if i_wrapped < 0 {
                    i_wrapped = rows as isize - 1;
                } else if i_wrapped >= rows as isize {
                    i_wrapped = 0;
                }
                if j_wrapped < 0 {
                    j_wrapped = cols as isize - 1;
                } else if j_wrapped >= cols as isize {
                    j_wrapped = 0;
                }
                let index = grid_index(i_wrapped as usize, j_wrapped as usize, cols);
                // Vérification et comptage des voisins vivants
                if !(i == row as isize && j == col as isize) && current_cells[index] >= 1 {
                    count += 1;
                }
            } else {
                // Si la grille n'est pas torique, on compte les voisins normalement
                if i >= 0
                    && i < rows as isize
                    && j >= 0
                    && j < cols as isize
                    && !(i == row as isize && j == col as isize)
                {
                    let index = grid_index(i as usize, j as usize, cols);
                    if current_cells[index] >= 1 {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

/// Additionne les valeurs des 8 cases autour d'une cellule spécifiée dans la grille.
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
/// La somme des valeurs des 8 cases autour de la cellule spécifiée.
///
/// # Exemple
///
/// ```
/// use crate::lifers::grid_get_value_neighbors_lenia;
///
/// let current_cells = vec![
///     1, 2, 3,
///     4, 5, 6,
///     7, 8, 9,
/// ];
/// let rows = 3;
/// let cols = 3;
/// let mut toricgrid = false;
///
/// // Somme des valeurs des cases autour de la cellule centrale
/// let mut sum_neighbors = grid_get_value_neighbors_lenia(1, 1, &current_cells, rows, cols, toricgrid);
/// assert_eq!(sum_neighbors, 40);
/// // Somme des valeurs des cases autour de 4
/// sum_neighbors = grid_get_value_neighbors_lenia(1, 0, &current_cells, rows, cols, toricgrid);
/// assert_eq!(sum_neighbors, 23);
/// toricgrid = true;
/// // Somme des valeurs des cases autour de 4 en toricgrid
/// sum_neighbors = grid_get_value_neighbors_lenia(1, 0, &current_cells, rows, cols, toricgrid);
/// assert_eq!(sum_neighbors, 41);
/// ```
pub fn grid_get_value_neighbors_lenia(
    row: usize,
    col: usize,
    current_cells: &[u8],
    rows: usize,
    cols: usize,
    toricgrid: bool,
) -> u16 {
    let mut sum = 0;

    // Parcours des cellules voisines de la cellule spécifiée
    for i in (row as isize - 1)..=(row as isize + 1) {
        for j in (col as isize - 1)..=(col as isize + 1) {
            if toricgrid {
                // Si la grille est torique, ajuste les indices des bords
                let mut i_wrapped = i;
                let mut j_wrapped = j;
                // Ajustement des indices pour les bords
                if i_wrapped < 0 {
                    i_wrapped = rows as isize - 1;
                } else if i_wrapped >= rows as isize {
                    i_wrapped = 0;
                }
                if j_wrapped < 0 {
                    j_wrapped = cols as isize - 1;
                } else if j_wrapped >= cols as isize {
                    j_wrapped = 0;
                }
                let index = grid_index(i_wrapped as usize, j_wrapped as usize, cols);
                // Addition des valeurs des voisins
                if !(i == row as isize && j == col as isize) {
                    sum += current_cells[index] as u16;
                }
            } else {
                // Si la grille n'est pas torique, on vérifie que les indices sont dans les limites de la grille
                if i >= 0 && i < rows as isize && j >= 0 && j < cols as isize {
                    let index = grid_index(i as usize, j as usize, cols);
                    // Addition des valeurs des voisins
                    if !(i == row as isize && j == col as isize) {
                        sum += current_cells[index] as u16;
                    }
                }
            }
        }
    }
    sum
}

/// Applique une convolution avec un filtre sur les valeurs des cases autour d'une cellule spécifiée dans la grille.
///
/// # Arguments
///
/// * `row` - L'indice de la ligne de la cellule dans la grille.
/// * `col` - L'indice de la colonne de la cellule dans la grille.
/// * `current_cells` - Vecteur contenant l'état actuel de chaque cellule de la grille.
/// * `rows` - Nombre de lignes de la grille.
/// * `cols` - Nombre de colonnes de la grille.
/// * `toricgrid` - Indique si les bords de la grille sont connectés, formant une grille torique.
/// * `filter` - Vecteur contenant les valeurs du filtre à appliquer.
///
/// # Returns
///
/// La somme pondérée des valeurs des cases autour de la cellule spécifiée en utilisant le filtre donné.
///
/// # Exemple
///
/// ```
/// use crate::lifers::grid_convolution_neighbors_lenia;
///
/// let current_cells = vec![
///     1, 2, 3,
///     4, 5, 6,
///     7, 8, 9,
/// ];
/// let rows = 3;
/// let cols = 3;
/// let mut toricgrid = false;
/// let mut filter = [
///     1, 1, 1,
///     1, 0, 1,
///     1, 1, 1,
/// ];
///
/// // Somme pondérée des valeurs des cases autour de la cellule centrale en utilisant le filtre donné
/// let mut weighted_sum = grid_convolution_neighbors_lenia(1, 1, &current_cells, rows, cols, toricgrid, &filter);
/// assert_eq!(weighted_sum, 40); // Exemple de résultat arbitraire
/// // Somme des valeurs des cases autour de 4
/// weighted_sum = grid_convolution_neighbors_lenia(1, 0, &current_cells, rows, cols, toricgrid, &filter);
/// assert_eq!(weighted_sum, 23);
/// toricgrid = true;
/// // Somme des valeurs des cases autour de 4 en toricgrid
/// weighted_sum = grid_convolution_neighbors_lenia(1, 0, &current_cells, rows, cols, toricgrid, &filter);
/// assert_eq!(weighted_sum, 41);
/// filter = [
///     1, 0, 1,
///     1, 0, 1,
///     1, 0, 1,
/// ];
/// // Somme pondérée des valeurs des cases autour de la cellule centrale en utilisant le filtre donné
/// weighted_sum = grid_convolution_neighbors_lenia(1, 1, &current_cells, rows, cols, toricgrid, &filter);
/// assert_eq!(weighted_sum, 30); // Exemple de résultat arbitraire
/// // Somme des valeurs des cases autour de 4
/// weighted_sum = grid_convolution_neighbors_lenia(1, 0, &current_cells, rows, cols, toricgrid, &filter);
/// assert_eq!(weighted_sum, 33);
/// toricgrid = false;
/// // Somme des valeurs des cases autour de 4 en non toricgrid
/// weighted_sum = grid_convolution_neighbors_lenia(1, 0, &current_cells, rows, cols, toricgrid, &filter);
/// assert_eq!(weighted_sum, 15);
/// ```
///
pub fn grid_convolution_neighbors_lenia(
    row: usize,
    col: usize,
    current_cells: &[u8],
    rows: usize,
    cols: usize,
    toricgrid: bool,
    filter: &[u8],
) -> u16 {
    let mut weighted_sum = 0;

    // Parcours des cellules voisines de la cellule spécifiée
    for i in (row as isize - 1)..=(row as isize + 1) {
        for j in (col as isize - 1)..=(col as isize + 1) {
            if toricgrid {
                // Si la grille est torique, ajuste les indices des bords
                let mut i_wrapped = i;
                let mut j_wrapped = j;
                // Ajustement des indices pour les bords
                if i_wrapped < 0 {
                    i_wrapped = rows as isize - 1;
                } else if i_wrapped >= rows as isize {
                    i_wrapped = 0;
                }
                if j_wrapped < 0 {
                    j_wrapped = cols as isize - 1;
                } else if j_wrapped >= cols as isize {
                    j_wrapped = 0;
                }
                let index = grid_index(i_wrapped as usize, j_wrapped as usize, cols);
                // Calcul de la somme pondérée en utilisant le filtre
                if !(i == row as isize && j == col as isize) {
                    weighted_sum += current_cells[index] as u16 * filter[(i - (row as isize) + 1) as usize * 3 + (j - (col as isize) + 1) as usize] as u16;
                }
            } else {
                // Si la grille n'est pas torique, on vérifie que les indices sont dans les limites de la grille
                if i >= 0 && i < rows as isize && j >= 0 && j < cols as isize {
                    let index = grid_index(i as usize, j as usize, cols);
                    // Calcul de la somme pondérée en utilisant le filtre
                    if !(i == row as isize && j == col as isize) {
                        weighted_sum += current_cells[index] as u16 * filter[(i - (row as isize) + 1) as usize * 3 + (j - (col as isize) + 1) as usize] as u16;
                    }
                }
            }
        }
    }

    weighted_sum
}


/// La fonction de croissance qui est appliqué par défaut à LeniaGrid
pub fn lenia_growth_function_default(value : u8) -> u8 {
    // return value
    return lenia_growth_function_gaussian(value, 122.0, 30.0)
}

/// Genere une gaussian
pub fn lenia_growth_function_gaussian(value: u8, center: f64, spread: f64) -> u8 {
    // Calcul de la valeur de la fonction de densité de probabilité gaussienne
    let gaussian_value = (-((value as f64 - center).powi(2) / (2.0 * spread.powi(2)))).exp() / (spread * (2.0 * PI).sqrt()); 
    // Conversion de la valeur de densité de probabilité à une échelle de 0 à 255
    let scaled_value = (gaussian_value * 255.0 * 100.0).round() as u8; // Multiplication par 100 car sinon nous perdons en precision à la conversion en u8

    scaled_value
}

/// Met à jour l'état de la grille selon les règles du jeu de la vie.
///
/// Cette fonction parcourt chaque cellule de la grille, compte ses voisins vivants et applique
/// les règles du jeu pour mettre à jour son état.
///
/// # Arguments
///
/// * `current_cells` - Vecteur contenant l'état actuel de chaque cellule de la grille.
/// * `next_cells` - Vecteur contenant l'état suivant de chaque cellule de la grille, utilisé
///   pour éviter de copier la grille inutilement à chaque mise à jour.
/// * `rows` - Nombre de lignes de la grille.
/// * `cols` - Nombre de colonnes de la grille.
/// * `toricgrid` - Indique si les bords de la grille sont connectés, formant une grille torique.
///
/// # Exemple
///
/// ```
/// use crate::lifers::grid_update_conways;
///
/// let mut current_cells = vec![
///     0, 0, 0,
///     2, 0, 2,
///     0, 0, 0,
/// ];
/// let mut next_cells = vec![0; 9];
/// let rows = 3;
/// let cols = 3;
/// let toricgrid = true;
///
/// // Met à jour l'état de la grille selon les règles du jeu de la vie
/// grid_update_conways(&mut current_cells, &mut next_cells, rows, cols, toricgrid);
/// ```
pub fn grid_update_conways(
    current_cells: &mut Vec<u8>,
    next_cells: &mut Vec<u8>,
    rows: usize,
    cols: usize,
    toricgrid: bool,
) {
    // Parcours de chaque cellule de la grille
    for row in 0..rows {
        for col in 0..cols {
            let current_index = grid_index(row, col, cols); // Calcul de l'index de la cellule actuelle
            let neighbors_count =
                grid_count_neighbors_conways(row, col, &current_cells, rows, cols, toricgrid); // Comptage des voisins vivants de la cellule actuelle
                                                                                       // Application des règles du jeu de la vie pour mettre à jour l'état de la cellule
            next_cells[current_index] = match (current_cells[current_index], neighbors_count) {
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
    std::mem::swap(current_cells, next_cells);
}

/// Met à jour l'état de la grille en appliquant une fonction de croissance sur la moyenne pondérée des valeurs des cellules voisines.
///
/// # Arguments
///
/// * `current_cells` - Vecteur contenant l'état actuel de chaque cellule de la grille.
/// * `next_cells` - Vecteur pour stocker le prochain état de la grille après la mise à jour.
/// * `rows` - Nombre de lignes de la grille.
/// * `cols` - Nombre de colonnes de la grille.
/// * `toricgrid` - Indique si les bords de la grille sont connectés, formant une grille torique.
/// * `filter` - Filtre pour pondérer les valeurs des cellules voisines.
/// * `neighbors_filter` - Nombre de voisins pris en compte dans le calcul de la moyenne pondérée.
/// * `growth_function` - Fonction de croissance à appliquer sur la moyenne pondérée des valeurs environnantes.
///
/// # Exemple
///
/// ```
/// use crate::lifers::{grid_update_lenia, grid_index, grid_convolution_neighbors_lenia};
///
/// // Définir les paramètres de la grille
/// let rows = 3;
/// let cols = 3;
/// let toricgrid = true;
/// let filter = vec![
///     1, 1, 1,
///     1, 0, 1,
///     1, 1, 1,
/// ];
/// let neighbors_filter = 8;
///
/// // Définir une fonction de croissance simple pour l'exemple
/// fn simple_growth_function(value: u8) -> u8 {
///     value + 1
/// }
///
/// // Créer des vecteurs pour stocker l'état actuel et le prochain état de la grille
/// let mut current_cells = vec![0; rows * cols];
/// let mut next_cells = vec![0; rows * cols];
///
/// // Appel de la fonction pour mettre à jour l'état de la grille (une grille remplie de 0)
/// grid_update_lenia(&mut current_cells, &mut next_cells, rows, cols, toricgrid, &filter, neighbors_filter, simple_growth_function);
///
/// // Vérification de l'état de la cellule centrale après la mise à jour
/// let central_index = grid_index(1, 1, cols); 
/// assert_eq!(current_cells[central_index], 1); // La valeur de la cellule centrale devrait être mise à jour selon la fonction de croissance
/// 
/// // Appel de la fonction pour mettre à jour l'état de la grille (une grille remplie de 1)
/// grid_update_lenia(&mut current_cells, &mut next_cells, rows, cols, toricgrid, &filter, neighbors_filter, simple_growth_function);
///
/// // Vérification de l'état de la cellule centrale après la mise à jour
/// let central_index = grid_index(1, 1, cols); 
/// assert_eq!(current_cells[central_index], 2); // La valeur de la cellule centrale devrait être mise à jour selon la fonction de croissance
/// ```
///
pub fn grid_update_lenia(
    current_cells: &mut Vec<u8>,
    next_cells: &mut Vec<u8>,
    rows: usize,
    cols: usize,
    toricgrid: bool,
    filter: &[u8], 
    neighbors_filter: usize,
    growth_function: fn(u8) -> u8,
) {
    // Parcours de chaque cellule de la grille
    for row in 0..rows {
        for col in 0..cols {
            let current_index: usize = grid_index(row, col, cols); // Calcul de l'index de la cellule actuelle
            let weighted_sum : u16 = grid_convolution_neighbors_lenia(row, col, current_cells, rows, cols, toricgrid, filter); // Calcul de la somme pondérée des valeurs des cellules voisines
            let average : u8 = (weighted_sum / neighbors_filter as u16) as u8; // Calcul de la moyenne des valeurs environnantes
            let updated_value : u8 = growth_function(average); // Application de la fonction de croissance sur la moyenne des valeurs environnantes
            next_cells[current_index] = updated_value; // Mise à jour de la valeur de la cellule dans le prochain état de la grille
        }
    }
    // Échange des vecteurs d'état actuel avec le prochain pour mettre à jour l'état de la grille
    std::mem::swap(current_cells, next_cells);
}

/// Dessine une grille.
///
/// Cette fonction prend un contexte mutable `ctx` de type `&mut Context`, une grille `grid` implémentant le trait `Grid`
/// et la taille de chaque cellule de la grille `cell_size`.
///
/// # Arguments
///
/// * `ctx` - Le contexte du jeu.
/// * `canvas` - Le canva sur lequel dessiner.
/// * `grid` - La grille à dessiner.
/// * `cell_size` - La taille de chaque cellule de la grille.
///
/// # Erreurs
///
/// Cette fonction peut retourner une erreur de type `GameError` si une erreur survient lors du dessin.
///
pub fn draw_grid<G: Grid>(ctx: &mut Context, canvas : &mut Canvas, grid: &G, cell_size: f32, color_alive : Option<Color>, color_not_alive : Option<Color>) -> GameResult {
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            let x = col as f32 * cell_size;
            let y = row as f32 * cell_size;
            let rect :  Rect = graphics::Rect::new(x, y, cell_size, cell_size);
            // graphics::Color::BLACK
            let color : Color = if grid_is_alive(row, col, &grid.current_cells(), grid.cols()) {
                color_alive.expect("Color for dead cells not found")
            } else { 
                color_not_alive.expect("Color for dead cells not found")
            };
            let mesh: graphics::Mesh =
                graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color)?;
            canvas.draw(&mesh, graphics::DrawParam::default());
            // graphics::draw(ctx, &mesh, graphics::DrawParam::default());
        }
    }
    // canvas.finish(ctx)?;
    Ok(())
}


/// Dessine une grille Lénia.
///
/// Cette fonction prend un contexte mutable `ctx` de type `&mut Context`, une grille `grid` implémentant le trait `Grid`
/// et la taille de chaque cellule de la grille `cell_size`.
///
/// # Arguments
///
/// * `ctx` - Le contexte du jeu.
/// * `canvas` - Le canva sur lequel dessiner.
/// * `grid` - La grille à dessiner.
/// * `cell_size` - La taille de chaque cellule de la grille.
/// * `cell_value_color_mappeur` - La fonction permettante de convertir la valeur de la cellule en une couleur
///
/// # Erreurs
///
/// Cette fonction peut retourner une erreur de type `GameError` si une erreur survient lors du dessin.
///
pub fn draw_grid_lenia<G: Grid>(ctx: &mut Context, canvas: &mut Canvas, grid: &G, cell_size: f32, cell_value_color_mappeur : fn(u8) -> Color) -> GameResult {
    let mut color : Color;
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            let x = col as f32 * cell_size;
            let y = row as f32 * cell_size;
            let rect : Rect =  graphics::Rect::new(x, y, cell_size, cell_size);

            let cell_value: u8 =  grid.current_cells()[grid_index(row, col, grid.cols())];
            // color = Color::from_rgba(cell_value, cell_value, cell_value, 255);
            color = cell_value_color_mappeur(cell_value);
            let mesh : Mesh =  graphics::Mesh::new_rectangle(ctx,  graphics::DrawMode::fill(), rect, color)?;
            canvas.draw(&mesh,  graphics::DrawParam::default());
        }
    }
    Ok(())
}

/// Fonction de mapping par défaut pour convertir la valeur de la cellule en couleur
pub fn map_cell_value_to_color(cell_value: u8) -> Color {
    // Calcul de la proportion de la valeur de cellule par rapport à la valeur maximale (255)
    let ratio = cell_value as f32 / 255.0;

    // Dégradé de couleur: Noir (0, 0, 0) à Violet (128, 0, 128) à Orange (255, 165, 0) à Jaune (255, 255, 0)
    let r = if ratio < 0.5 {
        (128.0 * ratio * 2.0) as u8 // Interpolation de 0 à 128 pour le rouge
    } else {
        255 // Orange
    };
    let g = if ratio < 0.5 {
        0 // Violet
    } else if ratio < 0.75 {
        (165.0 * (ratio - 0.5) * 4.0) as u8 // Interpolation de 0 à 165 pour le vert
    } else {
        165 // Orange
    };
    let b = if ratio < 0.5 {
        (128.0 * ratio * 2.0) as u8 // Interpolation de 0 à 128 pour le bleu
    } else {
        0 // Jaune
    };

    // Création et retour de la couleur correspondante
    Color::from_rgb(r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_count_neighbors_conways_toricgrid_enabled() {
        let current_cells = vec![0, 0, 0, 2, 0, 2, 0, 0, 0];
        let rows = 3;
        let cols = 3;
        let toricgrid = true;

        // Vérifie le nombre de voisins de la cellule en row 1 et colonne
        assert_eq!(
            grid_count_neighbors_conways(1, 1, &current_cells, rows, cols, toricgrid),
            2
        );
        // Vérifie le nombre de voisins de la cellule en haut à gauche
        assert_eq!(
            grid_count_neighbors_conways(0, 0, &current_cells, rows, cols, toricgrid),
            2
        );
        // Vérifie le nombre de voisins de la cellule en bas à droite
        assert_eq!(
            grid_count_neighbors_conways(2, 2, &current_cells, rows, cols, toricgrid),
            2
        );
    }

    #[test]
    fn test_grid_count_neighbors_conways_toricgrid_disabled() {
        let current_cells = vec![0, 0, 0, 2, 0, 2, 0, 0, 0];
        let rows = 3;
        let cols = 3;
        let toricgrid = false;

        // Vérifie le nombre de voisins de la cellule centrale en row 1 et colonne 1
        assert_eq!(
            grid_count_neighbors_conways(1, 1, &current_cells, rows, cols, toricgrid),
            2
        );
        // Vérifie le nombre de voisins de la cellule en haut à gauche
        assert_eq!(
            grid_count_neighbors_conways(0, 0, &current_cells, rows, cols, toricgrid),
            1
        );
        // Vérifie le nombre de voisins de la cellule en bas à droite
        assert_eq!(
            grid_count_neighbors_conways(2, 2, &current_cells, rows, cols, toricgrid),
            1
        );
    }
}
