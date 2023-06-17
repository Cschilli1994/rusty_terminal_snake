
pub mod game;
pub mod snake;
pub mod helper;
pub mod food;

use crate::game::Game;



fn main() {
    let mut game = Game::new();
    game.run();  
}
