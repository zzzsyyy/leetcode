struct Solution;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stk = vec![];
        let mut i = 0;
        for e in pushed.iter() {
            stk.push(e);
            while (!stk.is_empty() && stk[stk.len() - 1] == &popped[i]) {
                i += 1;
                stk.pop();
            }
        }
        return stk.is_empty();
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn s0946(){
        assert_eq!(Solution::validate_stack_sequences(vec![1,2,3,4,5], vec![4,5,3,2,1]), true);
        assert_eq!(Solution::validate_stack_sequences(vec![1,2,3,4,5], vec![4,3,5,1,2]), false);
    }
}
