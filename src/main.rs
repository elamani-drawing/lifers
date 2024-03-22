
use lifers::ConwaysGrid;
use lifers::Grid;
fn main() {
    println!("Hello, world!");
    let mut grid : ConwaysGrid = ConwaysGrid::new(5, 5, true);
    println!("{}",grid); 
    grid = ConwaysGrid::new_random(5, 5, true);
    println!("{}",grid); 
    grid.update();
    println!("{}",grid);  
}
