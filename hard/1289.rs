macro_rules! vecnd {
    ($([$($inner:tt)*]),+ $(,)?) => {
        vec![$(
            vecnd![$($inner)*]
        ),+]
    };
    ($($t:tt)*) => {
        vec![$($t)*]
    };
}

fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut dp = vec![vec![0; n]; 2];
    dp[0] = grid[0].clone();
    for i in 1..n {
        let v = i & 1;
        for j in 0..n {
            dp[v][j] = grid[i][j]
                + dp[1 - v]
                    .iter()
                    .enumerate()
                    .filter(|(k, v)| *k != j)
                    .map(|(_, e)| e)
                    .min()
                    .unwrap();
        }
    }
    *dp[(n - 1) & 1].iter().min().unwrap()
}

fn main() {
    // let m = [[2, 1, 3].to_vec(), [6, 5, 4].to_vec(), [7, 8, 9].to_vec()].to_vec();
    let g = vecnd![
        [-73, 61, 43, -48, -36],
        [3, 30, 27, 57, 10],
        [96, -76, 84, 59, -15],
        [5, -49, 76, 31, -7],
        [97, 91, 61, -46, 67]
    ];
    let ret = min_falling_path_sum(g);
    println!("{}", ret);
}
