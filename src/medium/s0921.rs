struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let (mut v, mut r) = (0i32, 0);
        s.chars().for_each(|c| {
            if c == ')' {
                v -= 1;
                if v < 0 {
                    r += v.abs();
                    v = 0;
                }
            } else {
                v += 1;
            }
        });
        r + v.abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s0921() {
        assert_eq!(Solution::min_add_to_make_valid("())".to_string()), 1);
        assert_eq!(Solution::min_add_to_make_valid("(((".to_string()), 3);
        assert_eq!(Solution::min_add_to_make_valid("(())".to_string()), 0);
    }
}
