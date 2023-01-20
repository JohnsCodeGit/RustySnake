use crate::snake_bit::SnakeBit;

pub struct Snake {
    pub bits: Vec<SnakeBit>
}

impl Snake {
    pub fn new() -> Snake
    {
        let mut snake = Snake {
            bits: Vec::new()
        };
        let snakeHead = SnakeBit::new_head();
        snake.bits.push(snakeHead);

        return snake;
    }

    pub fn get_head(&mut self) -> &mut SnakeBit
    {
        return &mut self.bits[0]; 
    }

    pub fn move_snake(&mut self)
    {
        
        for bit in (1..self.bits.len()).rev()
        {
            let x = self.bits[bit-1].get_x();
            let y = self.bits[bit-1].get_y();
            let dir = self.bits[bit-1].get_dir();
            self.bits[bit].set_dir(dir);
            self.bits[bit].set_pos(x,y);
            
        }

        self.get_head().go();
        
    }

    pub fn check_collision(&mut self) -> bool
    {
        
        println!("{0}",self.bits.len());
        for bit in 1..self.bits.len()
        {
            let x = self.bits[bit].get_x();
            let y = self.bits[bit].get_y();
            
            if x == self.get_head().get_x() && y == self.get_head().get_y()
            {
               
                return true;
            }
           
        }
        return false;
    }

    pub fn add_bit(&mut self)
    {
        let last_bit = self.bits.last().unwrap();

        let old_x = last_bit.get_x();
        let old_y = last_bit.get_y();
        let new_x;
        let new_y;
        //l1, r2, d3, u4
        match last_bit.get_dir()
        {
            1=>
            {
                new_x = old_x + 1;
                new_y = old_y;
            },
            2=>
            {
                new_x = old_x - 1;
                new_y = old_y;
            },
            3=>
            {
                new_x = old_x;
                new_y = old_y - 1;
            },
            4=>
            {
                new_x = old_x;
                new_y = old_y + 1;
            },
            _=>{
                new_x = old_x + 1;
                new_y = old_y + 1;
            }
            
        }

        let bit: SnakeBit = SnakeBit::new(last_bit.get_dir(), new_x, new_y);
        self.bits.push(bit);
    }

} 