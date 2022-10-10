impl Solution {
    pub fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let (mut dp1, mut dp2) = (0, 1);
        let (mut pre1, mut pre2) = (0, 1);
        for i in 1..n {
            dp1 = n;
            dp2 = n;
            if nums1[i] > nums1[i - 1] && nums2[i] > nums2[i - 1] {
                dp1 = pre1;
                dp2 = pre2 + 1;
            }
            if nums1[i] > nums2[i - 1] && nums2[i] > nums1[i - 1] {
                dp1 = dp1.min(pre2);
                dp2 = dp2.min(pre1 + 1);
            }
            pre1 = dp1;
            pre2 = dp2;
        }
        dp1.min(dp2) as i32
    }
}
