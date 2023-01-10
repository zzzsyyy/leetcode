use std::collections::HashSet;

impl Solution {
    pub fn num_components(mut head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        let mut set = nums.iter().collect::<HashSet<_>>();
        let (mut cur, mut res) = (0, 0);

        let mut head = &head;
        while head.is_some() {
            let cur_value = match head {
                Some(node) => node.val,
                None => -1,
            };
            if set.get(&cur_value).is_some() {
                if cur == 0 {
                    res += 1;
                }
                cur += 1;
            } else {
                cur = 0;
            }
            head = match head {
                Some(node) => &node.next,
                None => &None,
            };
        }
        res
    }
}
