struct Solution;

impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn s1626() {
        assert_eq!(
            Solution::best_team_score(vec![4, 5, 6, 5], vec![2, 1, 2, 1]),
            16
        );
    }
}
