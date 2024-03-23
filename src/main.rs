use ggez::{ContextBuilder, graphics::Color,};
use ggez::event;
use lifers::ConwaysGrid;
use lifers::Grid;
use lifers::LifeGui;

fn main() {
    // Make a Context.
    let (ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");
 
    let mut grid : ConwaysGrid = ConwaysGrid::new_random(100, 100, true);
    grid.set_color_alive(Some(Color::from_rgb(0, 0, 255)));
    grid.set_color_not_alive(Some(Color::from_rgb(0, 0, 0)));
    let mut my_game :LifeGui<ConwaysGrid> = LifeGui::new(grid.clone(), 8.);
    my_game.set_fps(3);
    event::run(ctx, event_loop, my_game);
}
