struct Solution;

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![0; prices.len()];
        let mut stk = vec![];
        for (e, n) in prices.iter().enumerate().rev() {
            while !stk.is_empty() && stk[stk.len() - 1] > n {
                stk.pop();
            }
            ret[e] = if stk.is_empty() {
                *n
            } else {
                n - stk[stk.len() - 1]
            };
            stk.push(n);
        }
        ret
    }
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_1475(){
        assert_eq!(Solution::final_prices(vec![8,4,6,2,3]), vec![4,2,4,2,3]);
    }
}
