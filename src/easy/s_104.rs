struct Solution;

use crate::utils::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let root = root.borrow();
            1 + Self::max_depth(root.left.clone()).max(Self::max_depth(root.right.clone()))
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let root = tree![3, 9, 20, null, null, 15, 7];
        assert_eq!(Solution::max_depth(root), 3);
    }
}
