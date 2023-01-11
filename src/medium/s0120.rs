struct Solution;

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

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn s0120(){
        assert_eq!(Solution::minimum_total(vecnd![[2],[3,4],[6,5,7],[4,1,8,3]]),11);
        assert_eq!(Solution::minimum_total(vecnd![[2],[3,4],[6,5,7],[4,1,8,3],[1,2,3,4,5]]),13);;
    }
}
