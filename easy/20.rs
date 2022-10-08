impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut vec = Vec::new();
        for i in s.chars() {
            match i {
                '(' | '{' | '[' => vec.push(i),
                _ => {
                    if vec.is_empty() {
                        return false;
                    } else {
                        let l = *vec.last().unwrap();
                        match &format!("{}{}", l, i)[..] {
                            "()" | "[]" | "{}" => vec.pop(),
                            _ => return false,
                        };
                    }
                }
            }
        }
        vec.is_empty()
    }
}
