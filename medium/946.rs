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
