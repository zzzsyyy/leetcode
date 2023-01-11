struct Solution;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let len = students.len();
        let mut cnt = vec![students.iter().filter(|&&x| x == 0).count()];
        cnt.push(len - cnt[0]);
        for (k, &v) in sandwiches.iter().enumerate() {
            if cnt[v as usize] == 0 {
                return (len - k) as i32;
            }
            cnt[v as usize] -= 1
        }
        0
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_1700(){
        assert_eq!(Solution::count_students(vec![1,1,0,0],vec![0,1,0,1]),0);
        assert_eq!(Solution::count_students(vec![1,1,1,0,0,1],vec![1,0,0,0,1,1]),3);
    }
}
