struct Solution;

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

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn s0915(){
        assert_eq!(Solution::partition_disjoint(vec![5,0,3,8,6]), 3);
        assert_eq!(Solution::partition_disjoint(vec![1,1,1,0,6,12]), 4);
    }
}
