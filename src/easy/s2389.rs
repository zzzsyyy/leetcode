struct Solution;

impl Solution {
    pub fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        nums.sort();
        let mut vec = vec![0;queries.len()];
        queries.iter().enumerate().for_each(|(i,&x)| {
            let mut sum = 0;
            for j in 0..nums.len() {
                sum += nums[j];
                if sum <= x {
                    vec[i] += 1;
                }else{
                    sum-=nums[j];
                }
            }
        });
        vec
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn s2389() {
        assert_eq!(
            Solution::answer_queries(vec![4, 5, 2, 1], vec![3, 10, 21]),
            vec![2, 3, 4]
        );
        assert_eq!(Solution::answer_queries(vec![2, 3, 4, 5], vec![1]), vec![0]);
    }
}
