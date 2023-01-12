struct Solution;

impl Solution {
    pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let len = nums1.len();
        let mut ret = vec![0; len];
        let mut nums1 = nums1;
        let mut nums2 = nums2.into_iter().enumerate().collect::<Vec<_>>();
        let (mut l, mut r) = (0, len - 1);
        nums1.sort();
        nums2.sort_by_key(|v| v.1);
        for i in 0..len {
            if nums1[i] > nums2[l].1 {
                ret[nums2[l].0] = nums1[i];
                l += 1;
            } else {
                ret[nums2[r].0] = nums1[i];
                r -= 1;
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn s0870(){
        assert_eq!(Solution::advantage_count(vec![2,7,11,15], vec![1,10,4,11]), vec![2,11,7,15]);
    }
}
