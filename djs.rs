#[derive(Debug)]
pub struct DJSet {
    p: Vec<usize>,
    r: Vec<usize>,
}

impl DJSet {
    pub fn new(n: usize) -> Self {
        Self {
            p: (1..=n).collect::<Vec<_>>(),
            r: vec![1; n],
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
        let (i, j) = (self.find(i), self.find(j));
        if r[i] <= r[j] {
            self.p[i] = j;
        } else {
            self.p[j] = i;
        }
        if r[i] == r[j] && i != j {
            r[j] += 1;
        }
    }
}
