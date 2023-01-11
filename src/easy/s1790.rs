struct Solution;

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        if s1.eq(&s2) {
            return true;
        }
        let (mut tmp1, mut tmp2) = ('\0', '\0');
        let mut cnt = 0;
        let s2 = s2.chars().collect::<Vec<_>>();
        for (k, v) in s1.chars().enumerate() {
            if s2[k] != v {
                cnt += 1;
                if tmp1 == '\0' {
                    tmp1 = v;
                    tmp2 = s2[k];
                } else if s2[k] != tmp1 || v != tmp2 {
                    return false;
                }
            }
        }
        cnt == 2
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn s1790(){
        assert_eq!(Solution::are_almost_equal("bank".to_string(),"kanb".to_string()),true);
        assert_eq!(Solution::are_almost_equal("attack".to_string(),"defend".to_string()),false);
        assert_eq!(Solution::are_almost_equal("kelb".to_string(),"kelb".to_string()),true);
        assert_eq!(Solution::are_almost_equal("abcd".to_string(),"dcba".to_string()),false);
    }
}
