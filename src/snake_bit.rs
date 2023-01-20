

pub struct SnakeBit
{
    X: i32,
    Y: i32,
    Dir: i32,
    char: char
}


impl SnakeBit 
{

    pub fn new(dir: i32, x: i32, y: i32) -> SnakeBit{
        SnakeBit{
            Dir: dir,
            X: x,
            Y: y,
            char: 'o'
        }
    }

    pub fn new_head() -> SnakeBit{
        SnakeBit{
            Dir: 3,
            X: 10,
            Y: 10,
            char: 'O'
        }
    }

    pub fn set_dir(&mut self, new_dir: i32)
    {
        self.Dir = new_dir;
    }
    pub fn set_pos(&mut self, new_x: i32, new_y: i32)
    {
 
        self.X = new_x;
        self.Y = new_y;
       
    }

    pub fn get_char(&self) -> char
    {
        return self.char
    }

    pub fn get_dir(&self) -> i32
    {  
        return self.Dir;
    }

    pub fn get_x(&self) -> i32
    {
        return self.X
    }

    pub fn get_y(&self) -> i32
    {
        return self.Y
    }

    pub fn go(&mut self)
    {
        match self.Dir
        {
            1 =>
            {
                if self.X > 3
                {
                    self.X -= 1
                }
            }
            2 =>
            {
                if self.X < 45
                {
                    self.X += 1
                }
            }
            3 =>
            {
                if self.Y < 16
                {
                    self.Y += 1
                }
            }
            4 =>
            {
                if self.Y > 1
                {
                    self.Y -= 1
                }
            }
            _ => {
                self.X = self.X;
                self.Y = self.Y - 1;
            }
            
        }
    }

}