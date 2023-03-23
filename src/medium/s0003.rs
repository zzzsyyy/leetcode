struct Solution;

impl Solution {
    fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let mut l = 0;
        let mut max = 0;
        let s = s.chars().collect::<Vec<_>>();
        for (r, &c) in s.iter().enumerate() {
            *map.entry(c).or_insert(0) += 1;
            while *map.get(&c).unwrap() > 1 {
                map.entry(s[l]).and_modify(|n| *n -= 1);
                l += 1;
            }
            max = max.max(r - l + 1);
        }
        max as i32
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn s0003() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }
}
