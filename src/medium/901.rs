struct StockSpanner {
    prices: Vec<i32>,
}

impl StockSpanner {
    fn new() -> Self {
        Self { prices: Vec::new() }
    }

    fn next(&mut self, price: i32) -> i32 {
        self.prices.push(price);

        let mut size = 0;

        for v in self.prices.iter().rev() {
            if *v <= price {
                size += 1;
            } else {
                break;
            }
        }

        size
    }
}
