impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::new();
        let (mut word1, mut word2) = (word1.chars(), word2.chars());
        loop {
            match (word1.next(), word2.next()) {
                (Some(c1), Some(c2)) => {
                    result.push(c1);
                    result.push(c2);
                }
                (Some(c1), None) => result.push(c1),
                (None, Some(c2)) => result.push(c2),
                _ => break,
            }
        }
        result
    }
}
