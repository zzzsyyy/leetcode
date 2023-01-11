struct Solution;

impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        let mut count =0;
        for i in 0..start_time.len(){
            if query_time <= end_time[i] && query_time >=start_time[i]{
                count+=1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_1450(){
        assert_eq!(Solution::busy_student(vec![1,2,3],vec![3,2,7],4),1);
    }
}
