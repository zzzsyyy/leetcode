use std::collections::HashMap;

impl Solution {
    pub fn find_max_k(mut nums: Vec<i32>) -> i32 {
        nums.sort_by(|a, b| b.cmp(a));
        let mut max = -1;
        let mut m = HashMap::new();
        for &i in nums.iter() {
            if i >= 0 {
                *m.entry(i).or_insert(0) += 1;
            } else {
                if let Some(_) = m.get(&-i) {
                    max = max.max(-i);
                }
            }
        }
        max
    }
}
