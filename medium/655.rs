use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        fn get_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> usize {
            if let Some(r) = root {
                let r = r.borrow();
                1 + get_depth(&r.left.clone()).max(get_depth(&r.right.clone()))
            } else {
                0
            }
        }
        fn dfs(
            root: &Option<Rc<RefCell<TreeNode>>>,
            vec: &mut Vec<Vec<String>>,
            row: usize,
            col: usize,
        ) {
            if let Some(root) = root {
                let cur = 1 << (vec.len() - row - 2);
                vec[row][col] = root.borrow().val.to_string();
                dfs(&root.borrow().left, vec, row + 1, col - cur);
                dfs(&root.borrow().right, vec, row + 1, col + cur);
            }
        }
        let row = get_depth(&root);
        let col = (1 << row) - 1;
        let mut vec: Vec<Vec<String>> = vec![vec!["".to_string(); col]; row];
        dfs(&root, &mut vec, 0, col / 2);
        vec
    }
}
