// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
type TreeLink = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_w = 0;
        fn dfs(
            r: &TreeLink,
            left: &mut Vec<usize>,
            level: usize,
            index: usize,
            mut max_w: i32,
        ) -> i32 {
            if r.is_none() {
                return max_w;
            }
            if level >= left.len() {
                left.push(index);
            }
            max_w = max_w.max(index as i32 - left[level] as i32 + 1);
            dfs(
                &r.as_ref().unwrap().borrow().left,
                left,
                level + 1,
                2 * index,
                max_w,
            )
            .max(dfs(
                &r.as_ref().unwrap().borrow().right,
                left,
                level + 1,
                2 * index + 1,
                max_w,
            ))
        }
        dfs(&root, &mut Vec::new(), 0, 1, 0)
    }
}
