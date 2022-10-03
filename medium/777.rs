impl Solution {
    pub fn can_transform(s: String, e: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let e: Vec<char> = e.chars().collect();
        let l = s.len();
        let (mut i, mut j) = (0, 0);
        loop {
            while i < l && s[i] == 'X' {
                i += 1;
            }
            while j < l && e[j] == 'X' {
                j += 1;
            }
            if i >= l && j >= l {
                return true;
            }
            if (i >= l || j >= l)
                || (s[i] != e[j])
                || (s[i] == 'R' && i > j)
                || (s[i] == 'L' && i < j)
            {
                return false;
            }
            i += 1;
            j += 1;
        }
        return true;
    }
}
