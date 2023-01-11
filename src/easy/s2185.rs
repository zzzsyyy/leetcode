struct Solution;

impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        let len = pref.len();
        let (mut count, mut f) = (0, 0);
        for s in words.iter(){
            let s = s.chars().collect::<Vec<_>>();
            if s.len() < len{continue;}
            for (i, c) in pref.chars().enumerate(){
                if c != s[i] {
                    f = 0;
                    break;
                }
                f=1
            }
            count+=f;
        }

        count as i32
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn s2185(){
        assert_eq!(Solution::prefix_count(vec!["a".to_string(),"b".to_string(),"c".to_string()], "a".to_string()),1);
        assert_eq!(Solution::prefix_count(vec!["a".to_string(),"b".to_string(),"c".to_string()], "b".to_string()),1);
        assert_eq!(Solution::prefix_count(vec!["a".to_string(),"b".to_string(),"c".to_string()], "ab".to_string()),0);
        assert_eq!(Solution::prefix_count(vec!["a".to_string(),"b".to_string(),"c".to_string()], "c".to_string()),1);
        assert_eq!(Solution::prefix_count(vec!["a".to_string(),"b".to_string(),"c".to_string()], "ac".to_string()),0);
        assert_eq!(Solution::prefix_count(vec!["a".to_string(),"b".to_string(),"c".to_string()], "bc".to_string()),0);
        assert_eq!(Solution::prefix_count(vec!["a".to_string(),"b".to_string(),"c".to_string()], "abc".to_string()),0);
    }
}
