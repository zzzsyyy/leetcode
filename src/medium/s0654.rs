struct Solution;

use crate::utils::tree_node::*;

impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> TreeLink {
        fn max_tree(nums: &Vec<i32>, l: i32, r: i32) -> TreeLink {
            if l > r {
                return None;
            }
            let max = *nums[l as usize..=r as usize].iter().max().unwrap();
            let cur = l + nums[l as usize..=r as usize]
                .iter()
                .position(|x| *x == max)
                .unwrap_or(0) as i32;
            let root = Rc::new(RefCell::new(TreeNode::new(max)));
            root.borrow_mut().left = max_tree(nums, l, cur - 1);
            root.borrow_mut().right = max_tree(nums, cur + 1, r);
            return Some(root);
        }
        return max_tree(&nums, 0, nums.len() as i32 - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn s0654() {
        assert_eq!(
            Solution::construct_maximum_binary_tree(vec![3, 2, 1, 6, 0, 5]),
            tree![6,3,5,null,2,0,null,null,1]
        );
    }
}
