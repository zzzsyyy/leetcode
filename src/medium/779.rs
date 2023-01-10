/* impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        let mut s = [0, 1].to_vec();
        if n == 2 {
            return s[k as usize - 1];
        }
        for _ in 2..n {
            let mut tmp = Vec::new();
            tmp.extend_from_slice(&s);
            let t = s.iter().map(|x| 1 - x).collect::<Vec<_>>();
            tmp.extend_from_slice(&t);
            s = tmp;
        }
        s[k as usize - 1]
    }
}
*/

/* impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
         (k - 1).count_ones() as i32 & 1
    }
}
*/

impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        return if n == 1 {
            0
        } else {
            kth_grammar(n - 1, k + 1 >> 1) ^ (k & 1) ^ 1
        };
    }
}
