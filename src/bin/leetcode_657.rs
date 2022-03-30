//! 机器人能否返回原点

pub fn judge_circle(moves: String) -> bool {
    let (mut l, mut r, mut d, mut u) = (0, 0, 0, 0);
    for &ch in moves.as_bytes() {
        match ch {
            b'L' => l += 1,
            b'R' => r += 1,
            b'U' => u += 1,
            b'D' => d += 1,
            _ => unreachable!(),
        }
    }
    l == r && d == u
}

fn main() {
    assert_eq!(judge_circle(String::from("UD")), true);
    assert_eq!(judge_circle(String::from("LL")), false);
}
