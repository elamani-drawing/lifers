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
    fn new(rows: usize, cols: usize, toricgrid: bool) -> Self ;

    
    /// Initialise une nouvelle grille du jeu de la vie avec des cellules dont l'état est aléatoire.
    ///
    /// # Arguments
    ///
    /// * `rows` - Le nombre de lignes de la grille.
    /// * `cols` - Le nombre de colonnes de la grille.
    /// * `toricgrid` - Le nombre de colonnes de la grille.
    ///
    fn new_random(rows: usize, cols: usize, toricgrid: bool) -> Self;

    // Méthode pour renvoyer le nombre de lignes de la grille
    fn rows(&self) -> usize;

    // Méthode pour renvoyer le nombre de colonnes de la grille
    fn cols(&self) -> usize;

    // Méthode pour indiquer si les bords de la grille sont connectés (forme une grille torique)
    fn is_toricgrid(&self) -> bool;

    // Méthode pour renvoyer une référence vers le vecteur contenant l'état actuel des cellules
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

    /// Inverse l'état d'une cellule spécifiée dans la grille.
    ///
    /// Si la cellule est vivante, elle devient morte, et si elle est morte, elle devient vivante.
    ///
    /// # Arguments
    ///
    /// * `row` - L'indice de la ligne de la cellule dans la grille.
    /// * `col` - L'indice de la colonne de la cellule dans la grille.
    ///
    fn toggle_cell_state(&mut self, row: usize, col: usize);

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
    fn index(&self, row: usize, col: usize) -> usize ;

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
}

