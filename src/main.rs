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


    if args.len() >= 2 && args[1] == "--rand" {

        let m1:types::Mx = gen_aleatory_2d_matrix(4, 10);
        let m2:types::Mx = gen_aleatory_2d_matrix(4, 10);
        
        let operation: char = '+';

        let ans:types::Mx = calc_matrix::add_2d(&m1, &m2);
        
        
        let m1_point: Point = Point { x: m1 };
        let m2_point: Point = Point { x: m2 };
        let ans_point: Point = Point { x: ans };

        print!("\n");
        println!("A:\n{}", m1_point);
        print!("{}\n\n", operation);
        println!("B:\n{}", m2_point);
        print!("=\n\n");
        println!("Result:\n{}", ans_point);
        return;
    } else {

        let mut len: String = String::new();
        let mut max: String = String::new();

        println!("Enter the length of the matrix: ");

        std::io::stdin().read_line(&mut len)
        .ok()
        .expect("Failed to read line");

        println!("Enter max value in the matrix: ");

        std::io::stdin().read_line(&mut max)
        .ok()
        .expect("Failed to read line");
        
        let len: i32 = len.trim().parse().unwrap();
        let max: i32 = max.trim().parse().unwrap();

        let m1:types::Mx = gen_aleatory_2d_matrix(len, max);
        let m2:types::Mx = gen_aleatory_2d_matrix(len, max);

        println!("Select an operation: \n1. Add (+)\n2. Subtract(-)\n3. Multiply (*)\n");

        let mut operation: String = String::new();

        std::io::stdin().read_line(&mut operation)
        .ok()
        .expect("Failed to read operation");

        let ans:types::Mx;

        match operation.trim() {
            "+" => ans = calc_matrix::add_2d(&m1, &m2),
            "-" => ans = calc_matrix::sub_2d(&m1, &m2),
            "*" => ans = calc_matrix::mult_2d(&m1, &m2),
            _ => {
                println!("Invalid operation");
                return;
            }
        }

        let m1_point: Point = Point { x: m1 };
        let m2_point: Point = Point { x: m2 };
        let ans_point: Point = Point { x: ans };
        
        print!("\n");
        println!("A:\n{}", m1_point);
        print!("{}\n\n", operation);
        println!("B:\n{}", m2_point);
        print!("=\n\n");
        println!("Result:\n{}", ans_point);
        return;
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