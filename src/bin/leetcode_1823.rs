//! 找出游戏的获胜者

pub fn find_the_winner(n: i32, k: i32) -> i32 {
    let mut s: Vec<i32> = (1..=n).collect();
    let mut cur = 0;
    let k = k as usize;
    while s.len() > 1 {
        cur = (cur + k - 1) % s.len();
        s.remove(cur);
    }
    s[0]
}

/// 约瑟夫环
pub fn find_the_winner_math(n: i32, k: i32) -> i32 {
    if n == 1 { return 1; }
    (k + find_the_winner_math(n - 1, k) - 1) % n + 1
}

/// 非递归版
pub fn find_the_winner_math2(n: i32, k: i32) -> i32 {
    let mut winner = 1;
    for i in 2..=n {
        winner = (k + winner - 1) % i + 1;
    }
    winner
}

fn main() {
    fn test(func: fn(n: i32, k: i32) -> i32) {
        assert_eq!(func(5, 2), 3);
        assert_eq!(func(6, 5), 1);
    }
    test(find_the_winner);
    test(find_the_winner_math);
    test(find_the_winner_math2);
}
