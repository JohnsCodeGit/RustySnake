pub use crate::snake::Snake;
pub use crate::food::Food;
use std::{thread, time};
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::time::Duration;

pub fn game_loop() {
    println!("The game will now begin.");

    let mut board = make_board();
    let mut snake = Snake::new();
    //snake.add_bit();
    let mut food = Food::new();
   

    loop {
        check_input(&mut snake);
        snake.move_snake();
        let game_over = snake.check_collision();
        if !game_over
        {
            draw(&mut board, &mut snake, &mut food);
            thread::sleep(time::Duration::from_millis(140));
            refresh(&mut board);
            board = make_board();
        }
        else 
        { 
            break;
        }
        
    }

}



pub fn check_input(snake: &mut Snake)
{
    let device_state = DeviceState::new();
    let snake_head = snake.get_head();
    let keys: Vec<Keycode> = device_state.get_keys();

    let dir = snake_head.get_dir();
    if !keys.is_empty(){
       
        if keys.contains(&Keycode::W) && dir != 3
        {
            snake_head.set_dir(4);
        }
        if keys.contains(&Keycode::A) && dir != 2
        {
            snake_head.set_dir(1);
        }
        if keys.contains(&Keycode::S) && dir != 4
        {
            snake_head.set_dir(3);
        }
        if keys.contains(&Keycode::D) && dir != 1
        {
            snake_head.set_dir(2);
        }
    }
    
}

pub fn refresh(board: &mut Vec<String>)
{   
    print!("\x1B[2J\x1B[1;1H");
    
    //std::process::Command::new("clear").status().unwrap();
    println!();
}

pub fn make_board() -> Vec<String>
{
    let mut board: Vec<String> = vec![String::new(); 18];

    for i in 0..18 {
        if i == 0 || i == 17
        {
            board[i] = String::from("-------------------------------------------------");
        }
        else 
        {
            board[i] = String::from("|                                               |");
        }
    }

    return board; 
}

pub fn draw(board: &mut Vec<String>, snake: &mut Snake, food: &mut Food)
{
   
    for bit in 0..snake.bits.len()
    {
       
        let mut temp_row_chars: Vec<char> = board[snake.bits[bit as usize].get_y() as usize].chars().collect();
        temp_row_chars[snake.bits[bit].get_x() as usize] = snake.bits[bit as usize].get_char();
        board[snake.bits[bit as usize].get_y() as usize] = temp_row_chars.iter().cloned().collect::<String>();
    
        if food.x == snake.bits[bit].get_x() as usize && food.y == snake.bits[bit].get_y() as usize 
        {
            food.next();
            snake.add_bit();
        }
        else
        {
            let temp_row_food = board[food.y].clone();
            let mut temp_row_food: Vec<char> = temp_row_food.chars().collect();
            temp_row_food[food.x] = '#';
            board[food.y] = temp_row_food.iter().cloned().collect::<String>();
        }
    }

    for i in 0..18 {
      
        println!("{0}", board[i]);
        
    }
}
