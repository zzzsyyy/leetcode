struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len()==1{
            return nums[0]
        }
        let mut num1 = nums[0];
        let mut num2 = nums[0].max(nums[1]);
        for n in nums.iter().skip(2) {
            let cur = (num1+n).max(num2);
            num1 = num2;
            num2 = cur;
        }
        num2
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn s0198(){
        assert_eq!(Solution::rob(vec![1,2,3,1]),4);
        assert_eq!(Solution::rob(vec![2,7,9,3,1]),12);
    }
}
