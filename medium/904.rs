use::collections::HashMap;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let n = fruits.len();
        let mut cnt = HashMap::new();
        let (mut i, mut ret) = (0,0);

        for j in 0..n {
            *cnt.entry(fruits[j]).or_insert(0) = j;
            if cnt.len() == 3 {
                if let Some((k, t)) = cnt.iter().map(|(&k, &v)| (v, k)).min() {
                    i = k + 1;
                    cnt.remove(&t);
                }
            }
            ret = ret.max(j - i + 1);
        }
        ret as i32
    }
}
