struct Solution;

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let kk = k.unsigned_abs() as usize;
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

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn s1652(){
        assert_eq!(Solution::decrypt(vec![5,7,1,4],3),vec![12,10,16,13]);
        assert_eq!(Solution::decrypt(vec![2,4,9,3],-2),vec![12,5,6,13]);
    }
}
