struct Solution;

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let len = search_word.len();
        let vec: Vec<&str> = sentence.as_str().split(" ").collect();
        for i in 0..vec.len() {
            if vec[i].len()<len{
                continue;
            }
            if vec[i][0..len] ==search_word{
                return i as i32 + 1;
            }
        }
        return -1;
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn s1455(){
        assert_eq!(Solution::is_prefix_of_word("i love eating burger".to_string(),"burg".to_string()),4);
    }
}
