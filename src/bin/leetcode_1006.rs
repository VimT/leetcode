//! 笨阶乘

pub fn clumsy(n: i32) -> i32 {
    match n {
        1 => return 1,
        2 => return 2,
        3 => return 6,
        _ => {}
    }
    let mut ans = 0;
    let mut add = n - 3;
    while add > 0 {
        ans += add;
        add -= 4;
    }
    ans += n * (n - 1) / (n - 2);
    let mut minus = n - 4;
    while minus > 0 {
        let mut tmp = minus;
        if minus - 1 > 0 { tmp *= minus - 1; }
        if minus - 2 > 0 { tmp /= minus - 2; }
        ans -= tmp;
        minus -= 4;
    }
    ans
}

fn main() {
    assert_eq!(clumsy(4), 7);
    assert_eq!(clumsy(10), 12);
}
