//! 完成所有交易的初始最少钱数

/// money = money - (cost - cashback)，要让每次做完交易的money尽可能小，亏损尽可能大
/// 所以是 按cashback升序，亏损降序排
/// 最后再选个盈利的最大cost
pub fn minimum_money(transactions: Vec<Vec<i32>>) -> i64 {
    let mut loss = vec![];
    let mut earning_max_cost = 0;
    for tran in transactions {
        if tran[0] > tran[1] {
            loss.push(tran);
        } else {
            earning_max_cost = earning_max_cost.max(tran[0]);
        }
    }
    loss.sort_unstable_by_key(|x| (x[1], (x[1] - x[0])));
    let mut init = 0;
    let mut cur = 0;
    for txn in loss {
        let cost = txn[0] as i64;
        let back = txn[1] as i64;
        if cur < cost {
            let diff = cost - cur;
            init += diff;
            cur += diff;
        }
        cur = cur - cost + back;
    }
    if cur < earning_max_cost as i64 {
        init += earning_max_cost as i64 - cur;
    }
    init
}

fn main() {
    assert_eq!(minimum_money(vec![vec![3, 9], vec![0, 4], vec![7, 10], vec![3, 5], vec![0, 9], vec![9, 3], vec![7, 4], vec![0, 0], vec![3, 3], vec![8, 0]]), 24);
    assert_eq!(minimum_money(vec![vec![2, 1], vec![5, 0], vec![4, 2]]), 10);
    assert_eq!(minimum_money(vec![vec![3, 0], vec![0, 3]]), 3);
}
