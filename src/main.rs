mod game;
mod timer;

fn main() {
    let mut game = game::Game::new();
    game.run();
}
