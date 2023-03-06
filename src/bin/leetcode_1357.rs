//! 每隔 n 个顾客打折

struct Cashier {
    n: i32,
    discount: i32,
    prices: Vec<i32>,
    cur: i32,
}


impl Cashier {
    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        let mut p = vec![0; 201];
        for (idx, price) in products.into_iter().zip(prices) {
            p[idx as usize] = price;
        }
        Self {
            n,
            discount,
            prices: p,
            cur: 0,
        }
    }

    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        self.cur += 1;
        let mut sum = 0;
        for (idx, amount) in product.into_iter().zip(amount) {
            sum += self.prices[idx as usize] * amount;
        }
        if self.cur % self.n == 0 {
            return (100 - self.discount) as f64 / 100. * sum as f64;
        }
        return sum as f64;
    }
}


fn main() {
    let mut cashier = Cashier::new(3, 50, vec![1, 2, 3, 4, 5, 6, 7], vec![100, 200, 300, 400, 300, 200, 100]);
    assert_eq!(cashier.get_bill(vec![1, 2], vec![1, 2]), 500.0);                        // 返回 500.0, 账单金额为 = 1 * 100 + 2 * 200 = 500.
    assert_eq!(cashier.get_bill(vec![3, 7], vec![10, 10]), 4000.0);                      // 返回 4000.0
    assert_eq!(cashier.get_bill(vec![1, 2, 3, 4, 5, 6, 7], vec![1, 1, 1, 1, 1, 1, 1]), 800.0);    // 返回 800.0 ，账单原本为 1600.0 ，但由于该顾客是第三位顾客，他将得到 50% 的折扣，所以实际金额为 1600 - 1600 * (50 / 100) = 800 。
    assert_eq!(cashier.get_bill(vec![4], vec![10]), 4000.0);                           // 返回 4000.0
    assert_eq!(cashier.get_bill(vec![7, 3], vec![10, 10]), 4000.0);                      // 返回 4000.0
    assert_eq!(cashier.get_bill(vec![7, 5, 3, 1, 6, 4, 2], vec![10, 10, 10, 9, 9, 9, 7]), 7350.0); // 返回 7350.0 ，账单原本为 14700.0 ，但由于系统计数再次达到三，该顾客将得到 50% 的折扣，实际金额为 7350.0 。
    assert_eq!(cashier.get_bill(vec![2, 3, 5], vec![5, 3, 2]), 2500.0);                    // 返回 2500.0
}
