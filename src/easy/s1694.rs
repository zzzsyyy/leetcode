struct Solution;

impl Solution {
    pub fn reformat_number(number: String) -> String {
        let mut ret = number
            .chars()
            .filter(|c| *c != ' ' && *c != '-')
            .collect::<String>();
        let mut n = ret.len();
        let mut i = 0;
        loop {
            match n - i {
                t if t > 4 => {
                    ret.insert(i + 3, '-');
                    i += 4;
                    n += 1;
                }
                4 => {
                    ret.insert(i + 2, '-');
                    break;
                }
                _ => break,
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn s1694(){
        assert_eq!(Solution::reformat_number("1-23-45 6".to_string()),"123-456".to_string());
    }
}
