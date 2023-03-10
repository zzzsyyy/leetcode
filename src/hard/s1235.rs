struct Solution;

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let l = profit.len();
        let mut idx = (0..end_time.len()).collect::<Vec<usize>>();
        idx.sort_unstable_by(|&a, &b| end_time[a].cmp(&end_time[b]));

        let mut dp = vec![0; l + 1];
        for i in 1..=l {
            let mut p = 0;
            for j in (1..i).rev() {
                if end_time[idx[j - 1]] <= start_time[idx[i - 1]] {
                    p = j;
                    break;
                }
            }
            dp[i] = dp[i - 1].max(dp[p] + profit[idx[i - 1]]);
        }
        dp[profit.len()]
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn s1235(){
        assert_eq!(Solution::job_scheduling(vec![1,2,3,3], vec![3,4,5,6], vec![50,10,40,70]), 120);
        assert_eq!(Solution::job_scheduling(vec![1,2,3,4,6], vec![3,5,10,6,9], vec![20,20,100,70,60]), 150);
        assert_eq!(Solution::job_scheduling(vec![1,1,1], vec![2,3,4], vec![5,6,4]), 6);
    }
}
