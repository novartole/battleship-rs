mod builder;
mod cmd;
mod field;
mod game;
mod seed;
mod ship;

use game::Game;

fn main() {
    let game = Game::new();
    game.setup();

    if let Err(e) = game.start() {
        println!("unexpected end of game: {}", e);
    }
}
