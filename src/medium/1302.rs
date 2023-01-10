use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
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
