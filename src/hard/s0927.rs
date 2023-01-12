struct Solution;

impl Solution {
    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        let cnt: i32 = arr.iter().sum();
        let mut a = 0;
        let (mut i, mut j, mut t) = (0usize, 0usize, 0usize);
        let l = arr.len();
        if cnt % 3 != 0 {
            return [-1, -1].to_vec();
        }
        if cnt == 0 {
            return [0, 2].to_vec();
        }
        arr.iter().enumerate().for_each(|(k, v)| {
            a += *v;
            match a {
                1 if *v == 1 => i = k,
                c if c == cnt / 3 + 1 && j == 0 => j = k,
                c if c == cnt / 3 * 2 + 1 && t == 0 => t = k,
                _ => {}
            }
        });
        let len = (l - t - 1) as usize;
        if i + len >= j || j + len >= t {
            return [-1, -1].to_vec();
        }
        for idx in 1..=len {
            let idx = idx as usize;
            if arr[i + idx] != arr[j + idx] || arr[j + idx] != arr[t + idx] {
                return [-1, -1].to_vec();
            }
        }
        [(i + len) as i32, (j + len + 1) as i32].to_vec()
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn s0927(){
        assert_eq!(Solution::three_equal_parts(vec![1,0,1,0,1]), vec![0, 3]);
        assert_eq!(Solution::three_equal_parts(vec![1,1,0,1,1]), vec![-1,-1]);
        assert_eq!(Solution::three_equal_parts(vec![1,1,0,0,1]), vec![0, 2]);
    }
}
