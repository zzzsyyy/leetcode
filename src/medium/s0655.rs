struct Solution;

use crate::utils::tree_node::*;

impl Solution {
    pub fn print_tree(root: TreeLink) -> Vec<Vec<String>> {
        fn get_depth(root: &TreeLink) -> usize {
            if let Some(r) = root {
                let r = r.borrow();
                1 + get_depth(&r.left.clone()).max(get_depth(&r.right.clone()))
            } else {
                0
            }
        }
        fn dfs(
            root: &TreeLink,
            vec: &mut Vec<Vec<String>>,
            row: usize,
            col: usize,
        ) {
            if let Some(root) = root {
                println!("{},{} ", row, vec.len() - row);
                //FIXME
                let cur:usize = 1 << (vec.len() - row - 2);
                vec[row][col] = root.borrow().val.to_string();
                dfs(&root.borrow().left, vec, row + 1, col - cur);
                dfs(&root.borrow().right, vec, row + 1, col + cur);
            }
        }
        let row = get_depth(&root);
        let col = (1 << row) - 1;
        let mut vec: Vec<Vec<String>> = vec![vec!["".to_string(); col]; row];
        dfs(&root, &mut vec, 0, col >> 1);
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn s0655() {
        assert_eq!(
            Solution::print_tree(tree![1, 2]),
            vecnd![["", "1", ""], ["2", "", ""]]
        );
        assert_eq!(
            Solution::print_tree(tree![1, 2, 3, null, 4]),
            vecnd![
                ["", "", "", "1", "", "", ""],
                ["", "2", "", "", "", "3", ""],
                ["", "", "4", "", "", "", ""]
            ]
        )
    }
}
