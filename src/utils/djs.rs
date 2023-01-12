#[derive(Debug)]
pub struct DJSet {
    p: Vec<usize>,
    r: Vec<usize>,
}

// set bound?
impl DJSet {
    pub fn new(n: usize) -> Self {
        Self {
            p: (0..=n).collect::<Vec<_>>(),
            r: vec![1; n + 1],
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
    /*
    pub fn merge(&mut self, i: usize, j: usize) {
        let idx = self.find(i);
        self.p[idx] = self.find(j);
    }
    */
    pub fn merge(&mut self, i: usize, j: usize) {
        let (i, j) = (self.find(i), self.find(j));
        if self.r[i] <= self.r[j] {
            self.p[i] = j;
        } else {
            self.p[j] = i;
        }
        if self.r[i] == self.r[j] && i != j {
            self.r[j] += 1;
        }
    }
}
