struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let len = nums.len();
        let mut count = 0_i32;
        let mut idx = len;
        let mut map = HashMap::new();
        let mut arr = vec![0; len + 1];
        nums.iter().enumerate().for_each(|(i, &n)| {
            arr[i + 1] = arr[i]
                + match n - k {
                    t if t > 0 => 1,
                    t if t < 0 => -1,
                    _ => {
                        idx = i;
                        0
                    }
                }
        });

        for i in 0..=len {
            if i <= idx {
                *map.entry(arr[i]).or_insert(0) += 1;
            } else {
                count += map.get(&arr[i]).unwrap_or(&0) + map.get(&(arr[i] - 1)).unwrap_or(&0);
            }
        }
        count
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn s2488() {
        assert_eq!(Solution::count_subarrays(vec![3, 2, 1, 4, 5], 4), 3);
        assert_eq!(
            Solution::count_subarrays(
                vec![
                    5, 19, 11, 15, 13, 16, 4, 6, 2, 7, 10, 8, 18, 20, 1, 3, 17, 9, 12, 14,
                ],
                6,
            ),
            13,
        );
    }
}
