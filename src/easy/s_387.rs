struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut cs = HashMap::new();
        s.chars().for_each(|c| *cs.entry(c).or_insert(0) += 1);
        for (i, c) in s.chars().enumerate() {
            if cs[&c] == 1 {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_387(){
        assert_eq!(Solution::first_uniq_char("leetcode".to_string()),0);
    }
}
