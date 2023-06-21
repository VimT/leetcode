//! 判断一个数是否迷人

pub fn is_fascinating(n: i32) -> bool {
    let s = n.to_string() + &(2 * n).to_string() + &(3 * n).to_string();
    let mut cnt = [0; 10];
    for &ch in s.as_bytes() {
        cnt[(ch - b'0') as usize] += 1;
    }
    cnt[0] == 0 && cnt[1..=9].iter().all(|&x| x == 1)
}

fn main() {
    fn test(func: fn(n: i32) -> bool) {
        assert_eq!(func(192), true);
        assert_eq!(func(100), false);
        assert_eq!(func(267), false);
        assert_eq!(func(783), false);
    }
    test(is_fascinating);
}
