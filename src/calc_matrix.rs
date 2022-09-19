type Mx = Vec<Vec<i32>>;

pub fn add_2d(m1: Mx, m2: Mx) -> Mx {
    let mut result: Mx = Vec::new();
    for i in 0..m1.len() {
        let mut row: Vec<i32> = Vec::new();
        for j in 0..m1[i].len() {
            row.push(m1[i][j] + m2[i][j]);
        }
        result.push(row);
    }
    result
}

pub fn sub_2d(m1: Mx, m2: Mx) -> Mx {
    let mut result: Mx = Vec::new();
    for i in 0..m1.len() {
        let mut row: Vec<i32> = Vec::new();
        for j in 0..m1[i].len() {
            row.push(m1[i][j] - m2[i][j]);
        }
        result.push(row);
    }
    result
}