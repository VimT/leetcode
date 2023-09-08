//! 取整购买后的账户余额

pub fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
    if purchase_amount % 10 >= 5 {
        100 - (purchase_amount + 10 - purchase_amount % 10)
    } else {
        100 - (purchase_amount - purchase_amount % 10)
    }
}

fn main() {
    fn test(func: fn(purchase_amount: i32) -> i32) {
        assert_eq!(func(9), 90);
        assert_eq!(func(15), 80);
    }
    test(account_balance_after_purchase);
}
