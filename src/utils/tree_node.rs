pub use std::cell::RefCell;
pub use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: TreeLink,
    pub right: TreeLink,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
pub type TreeLink = Option<Rc<RefCell<TreeNode>>>;

pub fn print_tree(root: TreeLink) {
    fn get_height(root: &TreeLink) -> usize {
        if let Some(r) = root {
            let r = r.borrow();
            1 + get_height(&r.left.clone()).max(get_height(&r.right.clone()))
        } else {
            0
        }
    }
    let height = get_height(&root);
    let width = (1 << height) - 1;
    let mut ans = vec![vec![" ".to_string(); width as usize]; height as usize];

    fn dfs(
        ans: &mut Vec<Vec<String>>,
        node: &TreeLink,
        deep: usize,
        lo: usize,
        hi: usize,
    ) {
        if let Some(x) = node {
            let node = x.borrow();
            let mid = lo + (hi - lo) / 2;
            ans[deep][mid] = x.borrow().val.to_string();
            dfs(ans, &node.left, deep + 1, lo, mid);
            dfs(ans, &node.right, deep + 1, mid + 1, hi);
        }
    }

    dfs(&mut ans, &root, 0usize, 0usize, width as usize);
    let ret = ans
        .iter()
        .map(|x| x.concat())
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", ret);
}

pub fn from_vec(v: Vec<Option<i32>>) -> TreeLink {
    use std::collections::VecDeque;
    let nodes: Vec<TreeLink> = v
        .iter()
        .map(|&node| {
            if node == None {
                return None;
            } else {
                Some(Rc::new(RefCell::new(TreeNode::new(node.unwrap()))))
            }
        })
        .collect();
    let mut queue = VecDeque::new();
    queue.push_back(0);
    let mut cursor = 1;
    while !queue.is_empty() {
        if let Some(index) = queue.pop_front() {
            if let Some(unwrapped_node) = nodes[index].clone() {
                let mut borrow_unwrapped_node = unwrapped_node.borrow_mut();
                if nodes.get(cursor).is_some() {
                    borrow_unwrapped_node.left = nodes[cursor].clone();
                    queue.push_back(cursor);
                }
                if nodes.get(cursor + 1).is_some() {
                    borrow_unwrapped_node.right = nodes[cursor + 1].clone();
                    queue.push_back(cursor + 1);
                }
                cursor += 2;
            }
        }
    }
    nodes[0].clone()
}

#[macro_export]
macro_rules! tree {
    () => {
        None
    };
    ($($e:expr),*) => {
        {
            let vec = vec![$(stringify!($e)), *];
            let vec = vec.into_iter().map(|v| v.parse::<i32>().ok()).collect::<Vec<_>>();
            from_vec(vec)
        }
    };
    ($($e:expr,)*) => {(tree![$($e),*])};
}
