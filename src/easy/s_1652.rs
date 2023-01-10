impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let kk = k.abs() as usize;
        let len = code.len();
        let mut ret = vec![0; len];
        if k == 0 {
            return ret;
        }
        for j in 0..len {
            let mut tail = if k > 0 {
                (j + 1) % len
            } else {
                (j + len - kk) % len
            };
            let mut sum = 0;
            for i in 0..kk {
                sum += code[tail];
                tail = (tail + 1) % len;
            }
            ret[j] = sum;
        }
        ret
    }
}
