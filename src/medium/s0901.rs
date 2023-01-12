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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn s0901() {
        let mut stock_spanner = StockSpanner::new();
        assert_eq!(stock_spanner.next(100), 1); 
        assert_eq!(stock_spanner.next(80), 1);
        assert_eq!(stock_spanner.next(60), 1);
        assert_eq!(stock_spanner.next(70), 2);
        assert_eq!(stock_spanner.next(60), 1);
        assert_eq!(stock_spanner.next(75), 4); // 返回 4 ，因为截至今天的最后 4 个股价 (包括今天的股价 75) 都小于或等于今天的股价。
        assert_eq!(stock_spanner.next(85), 6);
    }
}
