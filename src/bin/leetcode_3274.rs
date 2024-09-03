//! 检查棋盘方格颜色是否相同

pub fn check_two_chessboards(a: String, b: String) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    (a[0] as i32 + a[1] as i32) % 2 == (b[0] as i32 + b[1] as i32) % 2
}

fn main() {
    fn test(func: fn(coordinate1: String, coordinate2: String) -> bool) {
        assert_eq!(func(String::from("a1"), String::from("c3")), true);
        assert_eq!(func(String::from("a1"), String::from("h3")), false);
    }
    test(check_two_chessboards);
}
