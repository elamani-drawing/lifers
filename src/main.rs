
use lifers::Grid;
fn main() {
    println!("Hello, world!");
    let mut grid : Grid = Grid::new(5, 5, true);
    println!("{}",grid); 
    grid = Grid::new_random(5, 5, true);
    println!("{}",grid); 
    grid.update();
    println!("{}",grid); 
}
