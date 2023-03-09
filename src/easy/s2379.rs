struct Solution;

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let k = k as usize;
        let blocks = blocks.chars().collect::<Vec<_>>();
        let mut cnt = blocks.iter().take(k).filter(|x| **x == 'W').count();
        let mut ret = cnt;
        for (i, j) in (k..blocks.len()).enumerate() {
            if blocks[j] == 'W' {
                cnt += 1;
            }
            if blocks[i] == 'W' {
                cnt -= 1;
            }
            ret = ret.min(cnt);
        }
        ret as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn s2379() {
        assert_eq!(Solution::minimum_recolors("WBBWWBBWBW".to_string(), 7), 3);
        assert_eq!(Solution::minimum_recolors("WBWBBBW".to_string(), 2), 0);
    }
}
