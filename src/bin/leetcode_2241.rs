//! 设计一个 ATM 机器

struct ATM {
    cnt: [u64; 5],
}

static VAL: [u64; 5] = [20, 50, 100, 200, 500];

impl ATM {
    fn new() -> Self {
        Self { cnt: [0; 5] }
    }

    fn deposit(&mut self, banknotes_count: Vec<i32>) {
        for i in 0..5 {
            self.cnt[i] += banknotes_count[i] as u64;
        }
    }

    fn withdraw(&mut self, mut amount: i32) -> Vec<i32> {
        let mut result = vec![0; 5];
        for i in (0..5).rev() {
            if self.cnt[i] > 0 && amount as u64 >= VAL[i] {
                let cost = (amount as u64 / VAL[i]).min(self.cnt[i]);
                result[i] = cost as i32;
                amount -= (cost * VAL[i]) as i32;
            }
        }
        if amount != 0 {
            return vec![-1];
        }
        for i in 0..5 {
            self.cnt[i] -= result[i] as u64;
        }
        result
    }
}


fn main() {
    let mut atm = ATM::new();
    atm.deposit(vec![0, 0, 1, 2, 1]); // 存入 1 张 $100 ，2 张 $200 和 1 张 $500 的钞票。
    assert_eq!(atm.withdraw(600), vec![0, 0, 1, 0, 1]);        // 返回 [0,0,1,0,1] 。机器返回 1 张 $100 和 1 张 $500 的钞票。机器里剩余钞票的数量为 [0,0,0,2,0] 。
    atm.deposit(vec![0, 1, 0, 1, 1]); // 存入 1 张 $50 ，1 张 $200 和 1 张 $500 的钞票。 机器中剩余钞票数量为 [0,1,0,3,1] 。
    assert_eq!(atm.withdraw(600), vec![-1]);        // 返回 [-1] 。机器会尝试取出 $500 的钞票，然后无法得到剩余的 $100 ，所以取款请求会被拒绝。
    assert_eq!(atm.withdraw(550), vec![0, 1, 0, 0, 1]);        // 返回 [0,1,0,0,1] ，机器会返回 1 张 $50 的钞票和 1 张 $500 的钞票
}
