struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut vec = Vec::new();
        for i in s.chars() {
            match i {
                '(' | '{' | '[' => vec.push(i),
                _ => {
                    if vec.is_empty() {
                        return false;
                    } else {
                        let l = *vec.last().unwrap();
                        match &format!("{}{}", l, i)[..] {
                            "()" | "[]" | "{}" => vec.pop(),
                            _ => return false,
                        };
                    }
                }
            }
        }
        vec.is_empty()
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn s0020(){
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::is_valid("(".to_string()), false);
    }
}
