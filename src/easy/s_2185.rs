impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        let len = pref.len();
        let (mut count, mut f) = (0, 0);
        for s in words.iter(){
            let s = s.chars().collect::<Vec<_>>();
            if s.len() < len{continue;}
            for (i, c) in pref.chars().enumerate(){
                if c != s[i] {
                    f = 0;
                    break;
                }
                f=1
            }
            count+=f;
        }

        count as i32
    }
}
