//! 找到指定长度的回文数

pub fn kth_palindrome(queries: Vec<i32>, int_length: i32) -> Vec<i64> {
    let max = 10i64.pow(((int_length + 1) / 2) as u32) - 1;
    let start = 10i64.pow(((int_length + 1) / 2 - 1) as u32);
    queries.into_iter().map(|x| {
        let mut pre = start + x as i64 - 1;
        if pre > max {
            return -1;
        }
        let mut c = pre;
        if int_length & 1 == 1 {
            c /= 10;
        }
        while c > 0 {
            pre = pre * 10 + c % 10;
            c /= 10;
        }
        pre
    }).collect()
}

fn main() {
    fn test(func: fn(queries: Vec<i32>, int_length: i32) -> Vec<i64>) {
        assert_eq!(func(vec![1, 2, 3, 4, 5, 90], 3), vec![101, 111, 121, 131, 141, 999]);
        assert_eq!(func(vec![2, 4, 6], 4), vec![1111, 1331, 1551]);
    }
    test(kth_palindrome);
}
