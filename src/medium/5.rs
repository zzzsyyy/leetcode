// 二维动态规划
/* impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let v: Vec<char> = s.chars().collect();
        let l = v.len();
        let mut vec: Vec<Vec<bool>> = vec![vec![false; l]; l];
        let mut max = 1;
        let mut idx = 0;
        for i in 0..l {
            vec[i][i] = true;
        }
        for i in (0..l).rev() {
            for j in i..l {
                if v[i] == v[j] {
                    if j < i + 2 || vec[i + 1][j - 1] {
                        vec[i][j] = true;
                    }
                    if vec[i][j] && max < j - i + 1 {
                        max = j - i + 1;
                        idx = i;
                    }
                }
            }
        }
        s[idx..idx + max].to_string()
    }
} */

// 中心拓展法
/* impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        fn sh(s: &Vec<char>, mut l: i32, mut r: usize, n: usize) -> usize {
            let mut k = 0;
            while r < n && l >= 0 {
                if s[l as usize] == s[r] {
                    k += 1;
                    r += 1;
                    l -= 1;
                } else {
                    break;
                }
            }
            k
        }
        let v: Vec<char> = s.chars().collect();
        let n = v.len();
        let mut max = 1;
        let mut idx = 0;
        for i in 0..n {
            let k1 = sh(&v, i as i32 - 1, i + 1, n) * 2 + 1;
            let k2 = sh(&v, i as i32, i + 1, n) * 2;
            if max < k1 && k2 <= k1 {
                max = k1;
                idx = i - (k1 - 1) / 2;
            }
            if max < k2 && k1 <= k2 {
                max = k2;
                idx = i + 1 - k2 / 2;
            }
        }
        s[idx..idx + max].to_string()
    }
} */

// 一维动态规划
/* impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let v: Vec<char> = s.chars().collect();
        let n = v.len();
        let mut dp = vec![0; n];
        let mut max = 1;
        let mut idx = 0;
        for i in 1..n {
            if dp[i - 1] > 0 && v[i] == v[dp[i - 1] - 1] {
                dp[i] = dp[i - 1] - 1;
            } else {
                let mut r = i;
                let mut l = dp[i - 1];
                let mut st = l;
                while l < r {
                    if v[l] != v[r] {
                        r = i;
                        st = l + 1;
                    } else {
                        r -= 1;
                    }
                    l += 1;
                }
                dp[i] = st;
            }
            if max < i - dp[i] + 1 {
                idx = dp[i];
                max = i - dp[i] + 1;
            }
        }
        s[idx..idx + max].to_string()
    }
} */

// Manacher 方法
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let v: Vec<char> = s.chars().collect();
        let n = v.len();
        fn manacher_preprocess(s: &String) -> Vec<char> {
            let mut v: Vec<char> = vec!['|'];
            for c in s.chars() {
                v.push(c);
                v.push('|');
            }
            v
        }

        fn manacher(v: &Vec<char>, n: usize) -> (usize, usize) {
            let mut p: Vec<usize> = vec![1; n];
            let (mut max, mut idx) = (0, 0);
            let (mut l, mut r) = (0, 0);
            let mut k = 1;
            for i in 0..n {
                if i <= r {
                    k = (r - i + 1).min(p[l + r - i]);
                }
                while i >= k && i + k < n && v[i - k] == v[i + k] {
                    k += 1;
                }
                p[i] = k - 1;
                if max < p[i] {
                    max = p[i];
                    idx = (i - max) / 2
                }
                k -= 1;
                if i + k > r {
                    l = i - k;
                    r = i + k;
                }
            }
            (idx, max)
        }
        let v = manacher_preprocess(&s);
        let (idx, max) = manacher(&v, 2 * n + 1);
        s[idx..idx + max].to_string()
    }
}
