struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn crack_safe(n: i32, k: i32) -> String {
        if n == 1 && k == 1 {
            return "0".to_string();
        }
        let mut visited = HashSet::new();
        let mut ret = String::new();
        let start = "0".repeat((n - 1) as usize);
        find_euler_path(&mut visited, &mut ret, &start, n, k);
        fn find_euler_path(
            visited: &mut HashSet<String>,
            ret: &mut String,
            cur: &str,
            n: i32,
            k: i32,
        ) {
            for i in 0..k {
                let next = format!("{}{}", cur, i);
                let next = &next[next.len() - n as usize..];
                if !visited.contains(next) {
                    visited.insert(next.to_string());
                    find_euler_path(visited, ret, next, n, k);
                    ret.push_str(&i.to_string());
                }
            }
        }
        ret.push_str(&start);
        ret
    }
}

// Hierholzer 算法
// https://en.wikipedia.org/wiki/Eulerian_path
// https://en.wikipedia.org/wiki/Hierholzer%27s_algorithm
// https://www.jianshu.com/p/8394b8e5b878

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_753() {
        assert_eq!(Solution::crack_safe(1, 2), "10");
        assert_eq!(Solution::crack_safe(2, 2), "01100");
        Solution::crack_safe(2, 3);
    }
}
