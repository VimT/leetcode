//! 灯泡开关 Ⅱ

pub fn flip_lights(n: i32, presses: i32) -> i32 {
    let n = n.min(3) as usize;
    return match presses {
        0 => 1,
        1 => [2, 3, 4][n - 1],
        2 => [2, 4, 7][n - 1],
        _ => [2, 4, 8][n - 1],
    };
}

fn main() {
    assert_eq!(flip_lights(1, 1), 2);
    assert_eq!(flip_lights(2, 1), 3);
    assert_eq!(flip_lights(3, 1), 4);
}
