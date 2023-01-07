impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let res = nums.iter().sum::<i32>() - x;
        let len = nums.len();
        if res == 0 {
            return len as i32;
        }
        if res < 0 {
            return -1;
        }
        let (mut l, mut r, mut s, mut max) = (0, 0, 0, 0);
        while r < len && l <= r {
            s += nums[r];
            r += 1;
            while s > res {
                s -= nums[l];
                l += 1;
            }
            if s == res {
                max = max.max(r - l);
            }
        }
        return if max == 0 { -1 } else { (len - max) as i32 };
    }
}
