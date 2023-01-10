use std::collections::HashSet;

fn main() {
    let mut m = HashSet::new();
    let mut n = HashSet::new();
    let mut matrix = vec![vec![0,1,2,0], vec![3,4,5,2], vec![1,3,1,5]];
    // [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
    // let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];

    let a = matrix.len();
    let b = matrix[0].len();
    for i in 0..a {
        for j in 0..b {
            if matrix[i][j] == 0 {
                m.insert(i);
                n.insert(j);
            }
        }
    }
    println!("{:?}", m);
    for i in 0..a {
        for j in 0..b {
            if m.contains(&i) || n.contains(&j) {
                println!("{}|{}", i, j);
                matrix[i][j] = 0;
            }
        }
    }
    println!("{:?}", matrix);
}
