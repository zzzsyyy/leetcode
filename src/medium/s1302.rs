struct Solution;

use crate::utils::tree::*;

impl Solution {
    pub fn deepest_leaves_sum(root: TreeLink) -> i32 {
        let mut queue = std::collections::VecDeque::new();
        let root_rc = root.unwrap();
        queue.push_back(root_rc);
        let mut sum = 0;
        while !queue.is_empty() {
            sum = 0;
            for _ in 0..queue.len() {
                let node_rc = queue.pop_front().unwrap();
                let node = node_rc.borrow();
                sum += node.val;
                if let Some(left_rc) = &node.left {
                    queue.push_back(Rc::clone(left_rc));
                }
                if let Some(right_rc) = &node.right {
                    queue.push_back(Rc::clone(right_rc));
                }
            }
    }
    sum
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn s1302(){
        assert_eq!(Solution::deepest_leaves_sum(tree![1,2,3,4,5,null,6,7,null,null,null,null,8]), 15);
        assert_eq!(Solution::deepest_leaves_sum(tree![6,7,8,2,7,1,3,9,null,1,4,null,null,null,5]), 19);
    }
}
