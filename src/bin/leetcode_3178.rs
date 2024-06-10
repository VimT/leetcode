//! 找出 K 秒后拿着球的孩子

pub fn number_of_child(n: i32, k: i32) -> i32 {
    let mut cur = 0;
    let mut right = true;
    for _ in 0..k {
        if right {
            cur += 1;
            if cur == n {
                cur = n - 2;
                right = false;
            }
        } else {
            cur -= 1;
            if cur == -1 {
                cur = 1;
                right = true;
            }
        }
    }
    cur
}

/// 公式
pub fn number_of_child2(n: i32, k: i32) -> i32 {
    let (k, t) = (k / (n - 1), k % (n - 1));
    if k % 2 == 0 { t } else { n - 1 - t }
}

fn main() {
    fn test(func: fn(n: i32, k: i32) -> i32) {
        assert_eq!(func(3, 5), 1);
        assert_eq!(func(5, 6), 2);
        assert_eq!(func(4, 2), 2);
    }
    test(number_of_child);
    test(number_of_child2);
}
