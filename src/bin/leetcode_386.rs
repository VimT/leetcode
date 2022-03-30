//! 字典序排数

pub fn lexical_order(n: i32) -> Vec<i32> {
    fn dfs(n: i32, num: i32, result: &mut Vec<i32>) {
        if num > n { return; }
        result.push(num);
        for nxt in num * 10..num * 10 + 10 {
            dfs(n, nxt, result);
        }
    }
    let mut result = Vec::with_capacity(n as usize);
    for i in 1..10 {
        dfs(n, i, &mut result);
    }
    result
}

pub fn lexical_order_iter(n: i32) -> Vec<i32> {
    let mut result = Vec::with_capacity(n as usize);
    let mut num = 1;
    while result.len() < n as usize {
        while num <= n {
            result.push(num);
            num *= 10;
        }
        while num % 10 == 9 || num > n {
            num /= 10;
        }
        num += 1;
    }
    result
}

fn main() {
    assert_eq!(lexical_order_iter(13), vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(lexical_order_iter(2), vec![1, 2]);
}
