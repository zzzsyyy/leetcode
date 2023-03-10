struct Solution;

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut ret = 0;
        while x != 0 {
            let rett = ret * 10 + x % 10;
            if ((rett - x % 10) / 10) != ret {
                return 0;
            }
            ret = rett;
            x /= 10;
        }
        ret
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn s0007(){
        assert_eq!(Solution::reverse(123),321);
        assert_eq!(Solution::reverse(-123),-321);
    }
}
