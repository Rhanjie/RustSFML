mod game_manager;
mod custom_triangle;

use crate::game_manager::GameManager;

fn main() {
    let mut game_manager = GameManager::new(800, 600, "Rust + SFML");

    game_manager.init();
    game_manager.run();
}