use ggez::ContextBuilder;
use ggez::event;
use lifers::ConwaysGrid;
use lifers::Grid;
use lifers::LifeGui;

fn main() {
    // Make a Context.
    let (ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");
 
    let grid : ConwaysGrid = ConwaysGrid::new_random(50, 50, true);
    let my_game :LifeGui<ConwaysGrid> = LifeGui::new(grid.clone(), 20.);

    event::run(ctx, event_loop, my_game);
}
