struct Solution;

use crate::utils::djs::DJSet;

impl Solution {
    fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        let mut ds = DJSet::new(n * 2);
        for i in dislikes.iter() {
            let (a, b) = (i[0] as usize, i[1] as usize);
            if ds.find(a) == ds.find(b) {
                return false;
            } else {
                ds.merge(a, b + n);
                ds.merge(a + n, b);
            }
        }
        true
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn s0886(){
        assert_eq!(Solution::possible_bipartition(4, vec![vec![1,2], vec![1,3], vec![2,4]]), true);
        // assert_eq!
    }
}
