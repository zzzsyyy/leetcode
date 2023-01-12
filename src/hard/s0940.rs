struct Solution;

impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        let mut ab = vec![-1; 26];
        let mut cnt = vec![0, 0];
        let rem = 1_000_000_007;
        for (k, v) in s.chars().enumerate() {
            let t = v as usize - 'a' as usize;
            let i = k & 1;
            cnt[i] = cnt[1 - i] * 2 % rem - ab[t];
            cnt[i] += if cnt[i] < 0 { rem } else { 0 };
            ab[t] = cnt[1 - i] % rem;
        }
        cnt[1 - (s.len() & 1)]
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn s0940(){
        assert_eq!(Solution::distinct_subseq_ii("abc".to_string()), 7);
        assert_eq!(Solution::distinct_subseq_ii("aba".to_string()), 6);
        assert_eq!(Solution::distinct_subseq_ii("aaa".to_string()), 3);
    }
}
