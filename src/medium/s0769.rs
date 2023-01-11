struct Solution;

/* impl Solution {
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
*/

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut mask = 0;
        (0..arr.len())
            .filter(|&i| {
                mask |= 1 << arr[i];
                (mask & (mask + 1)) == 0
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn s0769(){
        assert_eq!(Solution::max_chunks_to_sorted(vec![4,3,2,1,0]), 1);
        assert_eq!(Solution::max_chunks_to_sorted(vec![1,0,2,3,4]), 4);
    }
}
