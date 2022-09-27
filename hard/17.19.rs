impl Solution {
    pub fn missing_two(nums: Vec<i32>) -> Vec<i32> {
        let mut t = 0_i32;
        let mut s = 0_i32;
        for i in 1..=nums.len() + 2 {
            t ^= i as i32;
        }
        for i in nums.iter() {
            t ^= i;
        }
        let lb = t & (-t);
        for i in nums.iter() {
            if (i & lb) != 0 {
                s ^= i;
            }
        }
        for i in 1..=nums.len() + 2 {
            if (i as i32 & lb) != 0 {
                s ^= i as i32;
            }
        }
        return vec![s, s ^ t];
    }
}
