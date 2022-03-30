//! 简易银行系统
struct Bank {
    b: Vec<i64>,
}


impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        Bank { b: balance }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        if (account1 as usize - 1) < self.b.len() && (account2 as usize - 1) < self.b.len() {
            if self.b[account1 as usize - 1] >= money {
                self.b[account1 as usize - 1] -= money;
                self.b[account2 as usize - 1] += money;
                return true;
            }
        }
        false
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        if (account as usize - 1) < self.b.len() {
            self.b[account as usize - 1] += money;
            return true;
        }
        false
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        if (account as usize - 1) < self.b.len() && self.b[account as usize - 1] >= money {
            self.b[account as usize - 1] -= money;
            return true;
        }
        false
    }
}

fn main() {
    let mut bank = Bank::new(vec![10, 100, 20, 50, 30]);
    assert_eq!(bank.withdraw(3, 10), true);    // 返回 true ，账户 3 的余额是 $20 ，所以可以取款 $10 。 账户 3 余额为 $20 - $10 = $10 。
    assert_eq!(bank.transfer(5, 1, 20), true); // 返回 true ，账户 5 的余额是 $30 ，所以可以转账 $20 。 账户 5 的余额为 $30 - $20 = $10 ，账户 1 的余额为 $10 + $20 = $30 。
    assert_eq!(bank.deposit(5, 20), true);     // 返回 true ，可以向账户 5 存款 $20 。 账户 5 的余额为 $10 + $20 = $30 。
    assert_eq!(bank.transfer(3, 4, 15), false); // 返回 false ，账户 3 的当前余额是 $10 。 所以无法转账 $15 。
    assert_eq!(bank.withdraw(10, 50), false);   // 返回 false ，交易无效，因为账户 10 并不存在。
}