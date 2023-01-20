use rand::Rng;
pub struct Food
{
    pub x: usize,
    pub y: usize
}

impl Food
{
    pub fn new() -> Food
    {
        let mut rng = rand::thread_rng();
        return Food {
            x: rng.gen_range(3..45),
            y: rng.gen_range(3..16)
        };
    }

    pub fn next(&mut self)
    {
        let mut rng = rand::thread_rng();
        self.x = rng.gen_range(3..45);
        self.y = rng.gen_range(3..16);

    }
}