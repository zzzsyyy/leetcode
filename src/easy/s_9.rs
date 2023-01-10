struct Solution;

impl Solution {
    pub fn is_palindrome_1(x: i32) -> bool {
        if x<0{return false}
        let mut x = x;
        let num = x;
        let mut y = 0;
        while (x!=0){
            y = y*10 + x%10;
            x/=10;
        }
        return num==y
    }
}

impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        if x<0 || (x%10==0&&x!=0){
            return false
        }
        let mut y = 0;
        while (x>y){
            y = y*10 + x%10;
            x/=10;
        }
        return x==y || x==y/10
    }
}

//反转数字的一半，直接来可能超出`i32::MAX`

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_9(){
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
        assert_eq!(Solution::is_palindrome_1(121), true);
        assert_eq!(Solution::is_palindrome_1(-121), false);
        assert_eq!(Solution::is_palindrome_1(10), false);
    }
}
