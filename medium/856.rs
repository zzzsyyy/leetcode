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
