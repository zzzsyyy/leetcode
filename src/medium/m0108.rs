struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut m = HashSet::new();
        let mut n = HashSet::new();
        let a = matrix.len();
        let b = matrix[0].len();
        for i in 0..a {
            for j in 0..b {
                if matrix[i][j] == 0 {
                    m.insert(i);
                    n.insert(j);
                }
            }
        }
        for i in 0..a {
            for j in 0..b {
                if m.contains(&i) || n.contains(&j) {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn m0108(){
        let mut m = vecnd![[1,1,1],[1,0,1],[1,1,1]];
        let mut n = vecnd![[0,1,2,0],[3,4,5,2],[1,3,1,5]];
        Solution::set_zeroes(&mut m);
        Solution::set_zeroes(&mut n);
        assert_eq!(m, vecnd![[1,0,1],[0,0,0],[1,0,1]]);
        assert_eq!(n, vecnd![[0,0,0,0],[0,4,5,0],[0,3,1,0]]);
    }
}
