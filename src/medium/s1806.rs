struct Solution;

impl Solution {
    pub fn reinitialize_permutation(n: i32) -> i32 {
        let perm = (0..n).collect::<Vec<i32>>();
        let n = n as usize;
        let mut count = 0i32;
        let (mut arr, mut tmp) = (perm.clone(), perm.clone());
        loop {
            for i in 0..n {
                arr[i] = if i % 2 == 0 {
                    tmp[i / 2]
                } else {
                    tmp[n / 2 + (i - 1) / 2]
                }
            }
            count += 1;
            if arr == perm {
                break;
            }
            tmp = arr.clone();
        }
        count
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn s1806(){
        assert_eq!(Solution::reinitialize_permutation(2), 1);
        assert_eq!(Solution::reinitialize_permutation(4), 2);
        assert_eq!(Solution::reinitialize_permutation(6), 4);
    }
}
