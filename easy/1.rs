use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m:HashMap<i32, i32> = HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            if let Some(&index) = m.get(&(target-v)) {
                return vec![index, i as i32];
            } else {
                m.entry(*v).or_insert(i as i32);
            }
        }
        vec![]
    }
}

//使用HashMap优化时间复杂度
//使用enumerate()方法，
//两个分支使用 if let
