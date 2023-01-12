struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut m = HashMap::new();
        for c in cpdomains.iter() {
            let split = c.split(' ').collect::<Vec<_>>();
            let num = split[0].parse::<usize>().unwrap();
            let url = split[1];
            m.entry(url.to_string())
                .and_modify(|x| *x += num)
                .or_insert(num);
            for (k, v) in url.as_bytes().iter().enumerate() {
                if *v == b'.' {
                    m.entry(url[k + 1..].to_string())
                        .and_modify(|x| *x += num)
                        .or_insert(num);
                }
            }
        }
        m.iter().map(|(x, y)| format!("{} {}", y, x)).collect()
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn s0811(){
        let cpdomains = vecstring!["900 google.mail.com", "50 yahoo.com", "1 intel.mail.com", "5 wiki.org"];
        let ret = vecstring!["951 com", "5 wiki.org", "50 yahoo.com", "1 intel.mail.com", "5 org", "901 mail.com", "900 google.mail.com"];
        assert_eq!(Solution::subdomain_visits(cpdomains), ret);
    }
}
