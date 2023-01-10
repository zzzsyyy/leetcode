struct Solution {}

impl Solution {
    fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut dp = vec![vec![0; n]; 2];
        dp[0] = grid[0].clone();
        for i in 1..n {
            let v = i & 1;
            let (idx, val) = dp[1 - v]
                .clone()
                .into_iter()
                .enumerate()
                .min_by(|a, b| a.1.cmp(&b.1))
                .unwrap();
            for j in 0..n {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1289() {
        let grid = vecnd![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        assert_eq!(Solution::min_falling_path_sum(grid), 13);
    }
}
