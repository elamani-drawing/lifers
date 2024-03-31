use ggez::{ContextBuilder, graphics::Color,};
use ggez::event;
use lifers::{ConwaysGrid, LeniaGrid};
use lifers::Grid;
use lifers::LifeGui;

fn main() {
    // Make a Context.
    let (ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");
 
    // let mut grid : ConwaysGrid = ConwaysGrid::new_random(100, 100, true);
    // grid.set_color_alive(Some(Color::from_rgb(0, 0, 255)));
    // grid.set_color_not_alive(Some(Color::from_rgb(0, 0, 0)));
    // let mut my_game :LifeGui<ConwaysGrid> = LifeGui::new(grid.clone(), 8.);

    // let mut grid : LeniaGrid = LeniaGrid::new_random(100, 100, true);
    
    let width = 100;
    let height = 100;
    let radius = 8;
    let center_x = width / 2;
    let center_y = height / 2;

    let matrix = degrade_create_matrix(width, height, center_x, center_y, radius);
    // print_matrix(&matrix, width);
    let mut grid : LeniaGrid = LeniaGrid::from_vect(matrix, 100, 100, true);
    // grid.set_filter(matrix);
    let mut my_game :LifeGui<LeniaGrid> = LifeGui::new(grid.clone(), 8.);
    my_game.set_fps(20);
    event::run(ctx, event_loop, my_game);
}




fn create_matrix(width: usize, height: usize, center_x: usize, center_y: usize, radius: usize) -> Vec<u8> {
    let mut matrix = vec![0; width * height];

    for y in 0..height {
        for x in 0..width {
            let index = y * width + x;
            let distance = ((x as isize - center_x as isize).pow(2) + (y as isize - center_y as isize).pow(2)) as f64;
            if distance <= radius.pow(2) as f64 {
                matrix[index] = 255;
            }
        }
    }

    matrix
}


fn degrade_create_matrix(width: usize, height: usize, center_x: usize, center_y: usize, radius: usize) -> Vec<u8> {
    let mut matrix = vec![0; width * height];

    for y in 0..height {
        for x in 0..width {
            let index = y * width + x;
            let distance = (((x as isize - center_x as isize).pow(2) + (y as isize - center_y as isize).pow(2)) as f64).sqrt();
            let value = (1.0 - (distance / radius as f64).min(1.0)) * 255.0;
            matrix[index] = value as u8;
        }
    }

    matrix
}

fn print_matrix(matrix: &[u8], width: usize) {
    for (i, &value) in matrix.iter().enumerate() {
        print!("{:4}", value);
        if (i + 1) % width == 0 {
            println!();
        }
    }
}