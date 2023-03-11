struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_longest_subarray(array: Vec<String>) -> Vec<String> {
        let len = array.len();
        let mut vec: Vec<i32> = vec![0; len + 1];
        array
            .iter()
            .enumerate()
            .for_each(|(i, x)| vec[i + 1] = vec[i] + if x.parse::<i32>().is_ok() { 1 } else { -1 });
        let mut map: HashMap<i32, usize> = HashMap::new();
        let (mut begin, mut end) = (0, 0);
        for i in 0..=len {
            if let Some(&s) = map.get(&vec[i]) {
                if end - begin < i - s {
                    (begin, end) = (s, i);
                };
            } else {
                map.insert(vec[i], i);
            }
        }
        array[begin..end].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn m1705() {
        assert_eq!(
            Solution::find_longest_subarray(vecstring![
                "A", "1", "B", "C", "D", "2", "3", "4", "E", "5", "F", "G", "6", "7", "H", "I",
                "J", "K", "L", "M"
            ]),
            vecstring!["A", "1", "B", "C", "D", "2", "3", "4", "E", "5", "F", "G", "6", "7"]
        );
        assert_eq!(
            Solution::find_longest_subarray(vecstring!["A", "A"]),
            Vec::<String>::new()
        );
    }
}
