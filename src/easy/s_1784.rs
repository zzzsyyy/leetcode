struct Solution;

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let mut d = 0;
        while d + 1 < s.len() {
            if s[d] == '0' && s[d + 1] == '1' {
                return false;
            }
            d += 1;
        }
        return true;
    }
}

#[cfg(test)]    
mod tests{
    use super::*;
    #[test]
    fn test_1784(){
        assert_eq!(Solution::check_ones_segment("1001".to_string()),false);
        assert_eq!(Solution::check_ones_segment("110".to_string()),true);
    }
}
