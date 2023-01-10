impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let (mut max, mut tmp, mut max_tmp) = (0, 0, 0);
        for &n in nums.iter() {
            if n > tmp {
                max_tmp += n;
            }
            if n <= tmp {
                max_tmp = n;
            }
            tmp = n;
            max = max.max(max_tmp);
        }
        max
    }
}
