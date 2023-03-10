struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let len = nums.len();
        let mut vec = vec![0; len + 1];
        nums.iter()
            .enumerate()
            .for_each(|(i, &n)| vec[i + 1] = (vec[i] + n % p) % p);
        let r = *vec.last().unwrap();
        if r == 0 {
            return 0;
        }
        let mut map = HashMap::new();
        let mut ret = len;
        //前缀和 优化两层循环
        for i in 0..=len {
            if let Some(j) = map.get(&vec[i]) {
                ret = ret.min(i - j);
            }
            map.insert((vec[i] + r) % p, i);
        }

        if ret == len {
            -1
        } else {
            ret as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s1590() {
        assert_eq!(Solution::min_subarray(vec![3, 1, 4, 2], 6), 1);
        assert_eq!(Solution::min_subarray(vec![6, 3, 5, 2], 9), 2);
        assert_eq!(Solution::min_subarray(vec![1, 2, 3], 3), 0);
        assert_eq!(Solution::min_subarray(vec![1, 2, 3], 7), -1);
        assert_eq!(
            Solution::min_subarray(vec![1000000000, 1000000000, 1000000000], 3),
            0
        );
        assert_eq!(Solution::min_subarray(vec![1, 2], 2), 1);
    }
}
