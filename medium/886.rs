
#[derive(Debug)]
pub struct DJSet {
    p: Vec<usize>,
}

impl DJSet {
    pub fn new(n: usize) -> Self {
        Self {
            p: (0..=n).collect::<Vec<_>>(),
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        if self.p[x] == x {
            x
        } else {
            let idx = self.p[x];
            self.p[x] = self.find(idx);
            self.p[x]
        }
    }
    pub fn merge(&mut self, i: usize, j: usize) {
        let idx = self.find(i);
        self.p[idx] = self.find(j);
    }
}

impl Solution {
    fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        let mut ds = DJSet::new(n * 2);
        for i in dislikes.iter() {
            let (a, b) = (i[0] as usize, i[1] as usize);
            if ds.find(a) == ds.find(b) {
                return false;
            } else {
                ds.merge(a, b + n);
                ds.merge(a + n, b);
            }
        }
        true
    }
}
