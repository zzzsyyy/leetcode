impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut dp = vec![vec![0; n]; 2];
        dp[0] = grid[0].clone();
        for (i,v) in grid.iter().skip(1).enumerate() {
            let t = 1-i & 1;
            let (idx, val) = dp[1 - t]
                .clone()
                .into_iter()
                .enumerate()
                .min_by(|a, b| a.1.cmp(&b.1))
                .unwrap();
            for (j, w) in v.iter().enumerate() {
                if j != idx {
                    dp[v][j] = val + grid[i][j];
                } else {
                    dp[v][j] = grid[i][j]
                        + dp[1 - v]
                            .iter()
                            .enumerate()
                            .filter(|(k, _)| *k != j)
                            .map(|(_, e)| e)
                            .min()
                            .unwrap();
                }
            }
        }
        *dp[(n - 1) & 1].iter().min().unwrap()
    }
}
