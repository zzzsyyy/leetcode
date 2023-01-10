use std::collections::HashMap;

impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut m = HashMap::new();
        for c in cpdomains.iter() {
            let split = c.split(" ").collect::<Vec<_>>();
            let num = split[0].parse::<usize>().unwrap();
            let url = split[1];
            m.entry(url.to_string())
                .and_modify(|x| *x += num)
                .or_insert(num);
            for (k, v) in url.as_bytes().iter().enumerate() {
                if *v == '.' as u8 {
                    m.entry(url[k + 1..].to_string())
                        .and_modify(|x| *x += num)
                        .or_insert(num);
                }
            }
        }
        m.iter().map(|(x, y)| format!("{} {}", y, x)).collect()
    }
}
