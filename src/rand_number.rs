use rand::Rng;

pub fn new(max: i32, zero: bool) -> i32 {
    let mut rng = rand::thread_rng();

    let number = rng.gen_range(0..max+1);
    
    if zero {
        return number;
    }else {
        return number + 1; 
    }
}
