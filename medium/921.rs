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
