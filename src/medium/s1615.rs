struct Solution;

impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut ret = 0;
        let mut is_link = vec![vec![false; n]; n];
        let mut vec = vec![0; n];
        roads.iter().for_each(|x| {
            let (m, n) = (*x.first().unwrap() as usize, *x.last().unwrap() as usize);
            is_link[m][n] = true;
            is_link[n][m] = true;
            vec[m] += 1;
            vec[n] += 1;
        });
        for i in 0..n {
            for j in i + 1..n {
                ret = ret.max(vec[i] + vec[j] - if is_link[i][j] { 1 } else { 0 });
            }
        }
        ret
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    fn s1615() {
        assert_eq!(
            Solution::maximal_network_rank(4, vecnd![[0, 1], [0, 3], [1, 2], [1, 3]]),
            4
        );
    }
}
