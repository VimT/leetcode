//! 银联-03. 理财产品

pub fn max_investment(product: Vec<i32>, limit: i32) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let mut left = 0;
    let mut right = *product.iter().max().unwrap();
    while left < right {
        let mid = (left + right) / 2;
        let mut cnt = 0;
        for &pro in &product {
            if pro >= mid {
                cnt += (pro - mid + 1) as i64;
            }
        }
        if cnt > limit as i64 {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    let mut cnt = 0;
    for &pro in &product {
        if pro >= left {
            cnt += (pro - left + 1) as i64;
        }
    }
    let mut result = 0;
    let mut more = limit as i64 - cnt;
    for pro in product {
        if pro >= left {
            let mut down = left;
            if left > 0 && more > 0 {
                more -= 1;
                down -= 1;
            }
            result = (result + (pro + down) as i64 * (pro - down + 1) as i64 / 2) % MOD;
        }
    }
    result as i32
}

fn main() {
    assert_eq!(max_investment(vec![2, 1, 3], 1), 3);
    assert_eq!(max_investment(vec![43877, 10848, 10442, 48132, 83395, 71523, 60275, 39527], 345056), 834376211);
    assert_eq!(max_investment(vec![4, 5, 3], 8), 26);
    assert_eq!(max_investment(vec![2, 1, 3], 20), 10);
}
