impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len()==1{
            return nums[0]
        }
        let mut num1 = nums[0];
        let mut num2 = nums[0].max(nums[1]);
        for i in 2..nums.len() {
            let cur = (num1+nums[i]).max(num2);
            num1 = num2;
            num2 = cur;
        }
        return num2
    }
}
