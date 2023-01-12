struct Solution;

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let (mut ret, mut n, mut tmp) = (0, 1, '\0');
        for c in s.chars() {
            if c == '(' {
                n <<= 1;
            }
            if c == ')' {
                if tmp == '(' {
                    ret += n / 2;
                }
                n >>= 1;
            }
            tmp = c;
        }
        ret
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn s0856(){
        assert_eq!(Solution::score_of_parentheses("()".to_string()), 1);
        assert_eq!(Solution::score_of_parentheses("(())".to_string()), 2);
        assert_eq!(Solution::score_of_parentheses("()()".to_string()), 2);
        assert_eq!(Solution::score_of_parentheses("(()(()))".to_string()), 6);
    }
}
