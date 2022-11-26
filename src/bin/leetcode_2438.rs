//! 二的幂数组中查询范围内的乘积

pub fn product_queries(mut n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut arr = vec![];
    while n > 0 {
        let num = n & -n;
        arr.push(num);
        n -= num;
    }
    const MOD: i64 = 1e9 as i64 + 7;
    queries.into_iter().map(|x| {
        arr[x[0] as usize..=x[1] as usize].iter().fold(1, |x, &y| (x as i64 * y as i64) % MOD) as i32
    }).collect()
}


fn main() {
    assert_eq!(product_queries(15, vec![vec![0, 1], vec![2, 2], vec![0, 3]]), vec![2, 4, 64]);
    assert_eq!(product_queries(2, vec![vec![0, 0]]), vec![2]);
}
