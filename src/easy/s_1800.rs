struct Solution;

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

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_1800(){
        assert_eq!(Solution::max_ascending_sum(vec![10,20,30,5,10,50]),65);
        assert_eq!(Solution::max_ascending_sum(vec![10,20,30,40,50]),150);
    }
}
