mod game_logic;
mod snake;
mod snake_bit;
mod food;

fn main() {
    println!("Hello, welcome to snake");
    game_logic::game_loop();
}
