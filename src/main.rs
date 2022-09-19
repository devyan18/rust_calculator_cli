use std::fmt;
use std::env;

mod rand_number;
mod types;
mod calc_matrix;

fn main() {
    struct Point {
        x: types::Mx,
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

        let oper = args[3].parse::<String>();


        if l.is_ok() && m.is_ok() {

            let ln = l.unwrap();
            let max = m.unwrap();
            
            
            let mx1 = gen_aleatory_2d_matrix(ln, max);
            let mx2 = gen_aleatory_2d_matrix(ln, max);
            
            let p1 = Point { x: mx1.clone()};
            let p2 = Point { x: mx2.clone()};
            
            let p3: Point;

            if oper == Ok("+".to_string()) {
                p3 = Point { x: calc_matrix::add_2d(mx1, mx2)};
                println!("{}\n + \n\n{}\n = \n\n{}", p1, p2, p3);
            } else {
                p3 = Point { x: calc_matrix::sub_2d(mx1, mx2)};
                println!("{}\n + \n\n{}\n = \n\n{}", p1, p2, p3);
            }

        } else {
            println!("Error: Invalid arguments");
        }
        
    } else {
        println!("Please enter a number");
    }
}



fn gen_aleatory_2d_matrix(len: i32, max: i32) -> types::Mx {
    let mut matrix: types::Mx = Vec::new();
    for _ in 0..len {
        let mut row: Vec<i32> = Vec::new();
        for _ in 0..len {
            row.push(rand_number::new(max, false));
        }
        matrix.push(row);
    }
    matrix
}