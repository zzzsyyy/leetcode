impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let mut s = s.chars().collect::<Vec<_>>();
        let mut d = vec![s[0] as i32 - 'a' as i32];
        for (idx, &c) in s.iter().enumerate().skip(1) {
            d.push(c as i32 - s[idx - 1] as i32);
        }
        d.push(0);
        for shift in shifts.iter() {
            let n = if shift[2] == 1 { 1 } else { -1 };
            d[shift[0] as usize] += n;
            d[shift[1] as usize + 1] -= n;
        }
        for i in 0..s.len() {
            d[i + 1] += d[i];
            s[i] = char::from_u32(((d[i] % 26 + 26) % 26) as u32 + 'a' as u32).unwrap();
        }
        s.iter().collect::<String>()
    }
}
