struct Solution;

impl Solution {
    pub fn build_array(target: Vec<i32>, _n: i32) -> Vec<String> {
        let p = &["Push".to_string()].to_vec();
        let q = &["Push".to_string(), "Pop".to_string()].to_vec();
        let mut ret = vec![];
        let mut m = 1;
        for n in target.into_iter() {
            while m < n {
                ret.extend_from_slice(q);
                m += 1;
            }
            ret.extend_from_slice(p);
            m += 1;
        }
        ret
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn s1441(){
        assert_eq!(Solution::build_array(vec![1,3], 3), vecstring!["Push", "Push", "Pop", "Push"]);
        assert_eq!(Solution::build_array(vec![1,2,3], 3), vecstring!["Push", "Push", "Push"]);
        assert_eq!(Solution::build_array(vec![1,2], 4), vecstring!["Push", "Push"]);
    }
}
