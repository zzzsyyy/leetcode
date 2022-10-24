impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let (mut left_max, mut curr_max, len, mut pos) = (nums[0], nums[0], nums.len(), 0usize);
        for i in 0..len - 1 {
            curr_max = curr_max.max(nums[i]);
            if nums[i] < left_max {
                left_max = curr_max;
                pos = i;
            }
        }
        pos as i32 + 1
    }
}
