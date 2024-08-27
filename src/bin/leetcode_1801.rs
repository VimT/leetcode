//! 积压订单中的订单总数

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// 简单的交易撮合系统
pub fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
    let mut sell: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();  // sell 要找最低的，小顶堆
    let mut buy: BinaryHeap<(i32, i32)> = BinaryHeap::new(); // buy 要找最高的，大顶堆
    for order in orders {
        let (price, mut amount, ty) = (order[0], order[1], order[2]);
        if ty == 0 {  // buy
            while amount > 0 && !sell.is_empty() && sell.peek().unwrap().0.0 <= price {
                let Reverse(mut sell_order) = sell.pop().unwrap();
                if sell_order.1 > amount {
                    sell_order.1 -= amount;
                    sell.push(Reverse(sell_order));
                    amount = 0;
                } else {
                    amount -= sell_order.1;
                }
            }
            if amount > 0 {
                buy.push((price, amount));
            }
        } else {  // sell
            while amount > 0 && !buy.is_empty() && buy.peek().unwrap().0 >= price {
                let mut buy_order = buy.pop().unwrap();
                if buy_order.1 > amount {
                    buy_order.1 -= amount;
                    buy.push(buy_order);
                    amount = 0;
                } else {
                    amount -= buy_order.1;
                }
            }
            if amount > 0 {
                sell.push(Reverse((price, amount)));
            }
        }
    }
    const MOD: i32 = 1_000_000_007;
    let mut result = 0;
    for Reverse((_, amount)) in sell {
        result = (result + amount) % MOD;
    }
    for (_, amount) in buy {
        result = (result + amount) % MOD;
    }
    result
}

fn main() {
    fn test(func: fn(orders: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![10, 5, 0], vec![15, 2, 1], vec![25, 1, 1], vec![30, 4, 0]]), 6);
        assert_eq!(func(vec![vec![7, 1000000000, 1], vec![15, 3, 0], vec![5, 999999995, 0], vec![5, 1, 1]]), 999999984);
    }
    test(get_number_of_backlog_orders);
}
