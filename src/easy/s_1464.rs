struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_by(|a, b| b.cmp(a));
        (nums[0]-1)*(nums[1]-1)
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_1464(){
        assert_eq!(Solution::max_product(vec![3,4,5,2]),12);
        assert_eq!(Solution::max_product(vec![1,5,4,5]),16);
    }
}
