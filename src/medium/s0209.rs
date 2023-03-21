struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut min = len + 1;
        let mut l = 0;
        let mut sum = 0;
        for (r, &x) in nums.iter().enumerate() {
            sum += x;
            while sum >= target {
                l += 1;
                sum -= nums[l];
                min = min.min(r - l + 1);
            }
        }
        if min <= len {
            min as i32
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn s0209() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]),2);
    }
}
