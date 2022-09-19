use rand::Rng;
use std::fmt;
use std::env;

fn main() {


    struct Point {
        x: Vec<Vec<i32>>,
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for i in 0..self.x.len() {
                for j in 0..self.x[i].len() {
                    write!(f, "{} ", self.x[i][j])?;
                }
                write!(f, "\n")?;
            }
            Ok(())
        }
    }

    let args: Vec<_> = env::args().collect();

    if args.len() >= 2 {
        let l = args[1].parse::<i32>();
        let m = args[2].parse::<i32>();

        if l.is_ok() && m.is_ok() {
            let x = Point { x: gen_aleatory_2d_matrix(l.unwrap(), m.unwrap()) };
            println!("{}", x);
        } else {
            println!("Error: Invalid arguments");
        }
        
    } else {
        println!("Please enter a number");
    }
}


fn gen_rand_number(max: i32, zero: bool) -> i32 {
    let mut rng = rand::thread_rng();

    let number = rng.gen_range(0..max+1);
    
    if zero {
        return number;
    }else {
        return number + 1; 
    }
}


fn gen_aleatory_2d_matrix(len: i32, max: i32) -> Vec<Vec<i32>> {
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for _ in 0..len {
        let mut row: Vec<i32> = Vec::new();
        for _ in 0..len {
            row.push(gen_rand_number(max, false));
        }
        matrix.push(row);
    }
    matrix
}
