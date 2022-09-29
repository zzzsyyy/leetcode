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
