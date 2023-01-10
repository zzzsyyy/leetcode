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
