impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let (mut tmp1, mut tmp2) = (0, 0);
        let mut cnt = 0;
        for (k, v) in arr.into_iter().enumerate() {
            tmp1 += k;
            tmp2 += v as usize;
            if tmp1 == tmp2 {
                cnt += 1;
            }
        }
        cnt
    }
}
