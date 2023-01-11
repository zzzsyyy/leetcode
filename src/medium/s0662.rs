struct Solution;

use crate::utils::tree_node::*;

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

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn s0662(){
        assert_eq!(Solution::width_of_binary_tree(tree![1,3,2,5,3,null,9]), 4);
        assert_eq!(Solution::width_of_binary_tree(tree![1,3,2,5,null,null,9,6,null,7]), 7);
        assert_eq!(Solution::width_of_binary_tree(tree![1,3,null,5,3]), 2);
    }
}
