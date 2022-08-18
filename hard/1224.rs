use std::collections::HashMap;

impl Solution {
    pub fn max_equal_freq(nums: Vec<i32>) -> i32 {
        let mut ret = 1;
        let mut max = 0;
        let mut map = HashMap::new();
        let mut feq = HashMap::new();
        for i in 1..=nums.len(){
            let c = map.entry(nums[i-1]).and_modify(|x| *x+=1).or_insert(1);
            if *c>1{*feq.entry(*c-1).or_default()-=1;}
            feq.entry(*c).and_modify(|x| *x+=1).or_insert(1);
            max=max.max(*c);
            if max == 1 
                || *feq.entry(max).or_default()*max+1 == i
                || *feq.entry(max-1).or_default()*(max-1)+max == i{
                ret = ret.max(i as i32);
            }
        }
        ret
    }
}
