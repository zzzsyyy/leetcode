impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let mut dp = vec![vec![0; n]; 2];
        dp[0] = matrix[0].clone();
        for (i, v) in matrix.iter().skip(1).enumerate() {
            let t = 1 - i & 1;
            for (j, w) in v.iter().enumerate() {
                if j == 0 {
                    dp[t][j] = dp[1 - t][j].min(dp[1 - t][j + 1]) + w;
                } else if j == n - 1 {
                    dp[t][j] = dp[1 - t][j].min(dp[1 - t][j - 1]) + w;
                } else {
                    dp[t][j] = dp[1 - t][j].min(dp[1 - t][j + 1]).min(dp[1 - t][j - 1]) + w;
                }
            }
        }
        *dp[(n - 1) & 1].iter().min().unwrap()
    }
}
