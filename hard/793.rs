impl Solution {
    pub fn preimage_size_fzf(mut k: i32) -> i32 {
        let mut t = 1;
        while t < k {
            t = t * 5 + 1;
        }
        while t > 1 {
            if t - 1 == k {
                return 0;
            }
            t = (t - 1) / 5;
            k %= t;
        }
        5
    }
}
