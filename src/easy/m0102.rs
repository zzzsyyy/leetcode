struct Solution;

impl Solution {
    pub fn check_permutation(s1: String, s2: String) -> bool {
        let mut s2:Vec<char>=s2.chars().collect();
        let mut s1:Vec<char>=s1.chars().collect();
        s1.sort();
        s2.sort();
        s1==s2
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn m0102(){
        assert_eq!(Solution::check_permutation("abc".to_string(),"bca".to_string()),true);
        assert_eq!(Solution::check_permutation("abc".to_string(),"bad".to_string()),false);
    }
}
