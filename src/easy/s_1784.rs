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
