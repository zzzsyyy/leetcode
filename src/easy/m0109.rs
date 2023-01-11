struct Solution;

impl Solution {
    pub fn is_fliped_string(s1: String, s2: String) -> bool {
        if s1.len()!=s2.len(){
            return false;
        }
        let s2 = s2.clone() + &s2;
        if let Some(r)=s2.find(&s1){
            return true;
        }
        return false;
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn m0109(){
        assert_eq!(Solution::is_fliped_string("waterbottle".to_string(),"erbottlewat".to_string()),true);
        assert_eq!(Solution::is_fliped_string("aa".to_string(),"aba".to_string()),false);
    }
}
