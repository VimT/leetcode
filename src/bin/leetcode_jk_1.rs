//! 九坤-01. 可以读通讯稿的组数

use std::collections::HashMap;

/// ra + b = a + rb, ra = a + x, rb = b + y。所以 x==y
pub fn number_of_pairs(nums: Vec<i32>) -> i32 {
    const MOD: i64 = 1e9 as i64 + 7;
    let mut map: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        let mut n = num;
        let mut reverse = 0;
        while n > 0 {
            reverse = reverse * 10 + n % 10;
            n /= 10;
        }
        *map.entry(reverse - num).or_default() += 1;
    }
    let mut result = 0;
    for (_, v) in map {
        result = (result + (v - 1) as i64 * v as i64 / 2) % MOD;
    }
    result as i32
}

fn main() {
    assert_eq!(number_of_pairs(vec![17, 28, 39, 71]), 3);
    assert_eq!(number_of_pairs(vec![71, 60]), 1);
}
