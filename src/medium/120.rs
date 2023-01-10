impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let m = triangle.len();
        let mut dp = vec![vec![0; m]; 2];
        dp[0][0] = triangle[0][0];
        for i in 1..m {
            for j in 0..=i {
                let v = triangle[i][j];
                if j > 0 && j < i {
                    dp[i & 1][j] = dp[(i - 1) & 1][j - 1].min(dp[(i - 1) & 1][j]) + v;
                } else if j == 0 {
                    dp[i & 1][j] = dp[(i - 1) & 1][j] + v;
                } else if j == i {
                    dp[i & 1][j] = dp[(i - 1) & 1][j - 1] + v;
                }
            }
        }
        *dp[(m - 1) & 1].iter().min().unwrap()
    }
}
