impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        let mut count = 0;
        for i in 0..=n {
            let mut a = i;
            let mut l = 1;
            while a != 0 {
                match a % 10 {
                    2 | 5 | 6 | 9 => a /= 10,
                    1 | 8 | 0 => {
                        l *= 10;
                        a /= 10;
                    }
                    _ => break,
                }
            }
            if i != 0 && a == 0 && i / l != 0 {
                count += 1;
            }
        }
        count
    }
}
