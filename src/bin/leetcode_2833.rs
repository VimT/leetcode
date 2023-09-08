//! 距离原点最远的点

pub fn furthest_distance_from_origin(moves: String) -> i32 {
    let mut l: i32 = 0;
    let mut r = 0;
    let mut x = 0;
    for &ch in moves.as_bytes() {
        match ch {
            b'L' => l += 1,
            b'R' => r += 1,
            b'_' => x += 1,
            _ => unreachable!()
        }
    }
    (l - r).abs() + x
}

fn main() {
    fn test(func: fn(moves: String) -> i32) {
        assert_eq!(func(String::from("L_RL__R")), 3);
        assert_eq!(func(String::from("_R__LL_")), 5);
        assert_eq!(func(String::from("_______")), 7);
    }
    test(furthest_distance_from_origin);
}
