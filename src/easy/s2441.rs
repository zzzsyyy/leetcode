struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_max_k(mut nums: Vec<i32>) -> i32 {
        nums.sort_by(|a, b| b.cmp(a));
        let mut max = -1;
        let mut m = HashMap::new();
        for &i in nums.iter() {
            if i >= 0 {
                *m.entry(i).or_insert(0) += 1;
            } else if m.get(&-i).is_some() {
                    max = max.max(-i);
                }
        }
        max
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn s2441(){
        assert_eq!(Solution::find_max_k(vec![1,2,3,4]),-1);
        assert_eq!(Solution::find_max_k(vec![3,2,-2,5,-3]),3);
    }
}
