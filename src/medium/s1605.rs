struct Solution;

impl Solution {
    pub fn restore_matrix(row_sum: Vec<i32>, col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let (m, n) = (row_sum.len(), col_sum.len());
        let mut ret = vec![vec![0; n]; m];
        let (mut row_sum, mut col_sum) = (row_sum, col_sum);
        let (mut i, mut j) = (0, 0);
        while i < m && j < n {
            if row_sum[i] < col_sum[j] {
                ret[i][j] = row_sum[i];
                col_sum[j] -= row_sum[i];
                i += 1;
            } else {
                ret[i][j] = col_sum[j];
                row_sum[i] -= col_sum[j];
                j += 1;
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s1605() {
        assert_eq!(
            Solution::restore_matrix(vec![3, 8], vec![4, 7]),
            vecnd![[3, 0], [1, 7]]
        );
        assert_eq!(
            Solution::restore_matrix(vec![5, 7, 10], vec![8, 6, 8]),
            vecnd![[5, 0, 0], [3, 4, 0], [0, 2, 8]]
        );
        assert_eq!(
            Solution::restore_matrix(vec![14, 9], vec![6, 9, 8]),
            vecnd![[6, 8, 0], [0, 1, 8]]
        );
    }
}
