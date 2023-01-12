struct Solution;

impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        let m = digits.len();
        let digits = digits
            .iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let s = n
            .to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>();
        let n = s.len() + 1;
        let mut dp = vec![vec![0; n]; n];
        dp[0][1] = 1;
        for (k, v) in s.iter().enumerate() {
            for d in digits.iter() {
                if d == v {
                    dp[k + 1][1] = dp[k][1];
                } else if d < v {
                    dp[k + 1][0] += dp[k][1];
                } else {
                    break;
                }
            }
            if k > 0 {
                dp[k + 1][0] += m + dp[k][0] * m
            }
        }
        (dp[n - 1][0] + dp[n - 1][1]) as i32
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn s0902(){
        assert_eq!(Solution::at_most_n_given_digit_set(vecstring!["1","3","5","7"], 100), 20);
        assert_eq!(Solution::at_most_n_given_digit_set(vecstring!["1","4","9"], 1000000000), 29523);
        assert_eq!(Solution::at_most_n_given_digit_set(vecstring!["7"], 8), 1);
    }
}
