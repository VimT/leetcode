//! 完美表演

pub fn perfect_performance(s: String) -> bool {
    let (mut l, mut r, mut u, mut d) = (0, 0, 0, 0);
    for &ch in s.as_bytes() {
        match ch {
            b'L' => l += 1,
            b'R' => r += 1,
            b'U' => u += 1,
            b'D' => d += 1,
            _ => unreachable!()
        }
    }
    l == r && u == d
}

fn main() {
    fn test(func: fn(s: String) -> bool) {
        assert_eq!(func(String::from("UD")), true);
        assert_eq!(func(String::from("LL")), false);
    }
    test(perfect_performance);
}
