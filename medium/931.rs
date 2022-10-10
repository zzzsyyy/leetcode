fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let n = matrix.len();
    let mut dp = vec![vec![0; n]; 2];
    dp[0] = matrix[0].clone();
    for i in 1..n {
        let v = i & 1;
        for j in 0..n {
            if j == 0 {
                dp[v][j] = dp[1 - v][j].min(dp[1 - v][j + 1]) + matrix[i][j];
            } else if j == n - 1 {
                dp[v][j] = dp[1 - v][j].min(dp[1 - v][j - 1]) + matrix[i][j];
            } else {
                dp[v][j] = dp[1 - v][j].min(dp[1 - v][j + 1]).min(dp[1 - v][j - 1]) + matrix[i][j];
            }
        }
    }
    *dp[(n - 1) & 1].iter().min().unwrap()
}

fn main() {
    let m = [[2, 1, 3].to_vec(), [6, 5, 4].to_vec(), [7, 8, 9].to_vec()].to_vec();
    let ret = min_falling_path_sum(m);
    println!("{}", ret);
}
