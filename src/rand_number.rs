use rand::{Rng, rngs::ThreadRng};

pub fn new(max: i32, zero: bool) -> i32 {

    const INITIAL_RANGE: i32 = 0;
    const INCLUSIVE_MAX_RANGE: i32 = 1;

    let final_range: i32 = max + INCLUSIVE_MAX_RANGE;

    let mut rng: ThreadRng = rand::thread_rng();

    let number: i32 = rng.gen_range(INITIAL_RANGE..final_range);
    
    if zero {
        return number;
    }
    
    number + 1 
}
