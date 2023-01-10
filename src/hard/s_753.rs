struct Solution;

impl Solution {
    pub fn crack_safe(n: i32, k: i32) -> String {
        
    }
}

#[cfg(test)]
fn test(){
    use siper::*;

    #[test]
    fn test_753() {
        assert_eq!(Solution::crack_safe(1, 2), "01");
        assert_eq!(Solution::crack_safe(2, 2), "00110");
    }
}
