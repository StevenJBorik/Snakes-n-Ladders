use rand::Rng; 

// 12/31 7:35 
#[derive(Debug)]
pub struct Dice {
    minValue: i32,
    maxValue: i32,
    totalRollValue: i32,
}

impl Dice {
    pub fn roll(mut self) -> Dice {
        let mut rng = rand::thread_rng(); 
        return rng.gen_range(minValue..maxValue + 1); 
    }
}

