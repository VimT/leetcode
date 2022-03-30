//! 股票价格波动

use std::collections::{BTreeMap, HashMap};

struct StockPrice {
    tree: BTreeMap<i32, i32>,
    tp: HashMap<i32, i32>,
    last_t: i32,
    last_p: i32,
}


impl StockPrice {
    fn new() -> Self {
        StockPrice { tree: BTreeMap::new(), tp: HashMap::new(), last_t: 0, last_p: 0 }
    }

    fn update(&mut self, timestamp: i32, price: i32) {
        if let Some(&v) = self.tp.get(&timestamp) {
            let count = *self.tree.get(&v).unwrap();
            if count == 1 {
                self.tree.remove(&v);
            } else {
                self.tree.insert(v, count - 1);
            }
        }
        *self.tree.entry(price).or_insert(0) += 1;
        if timestamp >= self.last_t {
            self.last_t = timestamp;
            self.last_p = price;
        }
        self.tp.insert(timestamp, price);
    }

    fn current(&self) -> i32 {
        return self.last_p;
    }

    fn maximum(&self) -> i32 {
        *self.tree.range(..).last().unwrap().0
    }

    fn minimum(&self) -> i32 {
        *self.tree.range(..).next().unwrap().0
    }
}

fn main() {
    let mut sp = StockPrice::new();
    sp.update(1, 10);
    sp.update(2, 5);
    println!("{}", sp.current()); // 5
    println!("{}", sp.maximum()); //10
    sp.update(1, 3);
    println!("{}", sp.maximum()); // 5
    sp.update(4, 2);
    println!("{}", sp.minimum()); //2
}