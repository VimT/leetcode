//! 困于环中的机器人

/// 一次指令之后，只有(x,y)不是原点，并且方向和原来的方向一致，最后才回不去
pub fn is_robot_bounded(instructions: String) -> bool {
    let mut d = 0;
    let dirs = [[1, 0], [0, 1], [-1, 0], [0, -1]];
    let mut x = 0;
    let mut y = 0;
    for &ins in instructions.as_bytes() {
        match ins {
            b'R' => { d += 1; }
            b'L' => { d += 3; }
            b'G' => {
                d %= 4;
                x += dirs[d][0];
                y += dirs[d][1];
            }
            _ => unreachable!()
        }
    }
    (x == 0 && y == 0) || (d % 4 != 0)
}

fn main() {
    assert_eq!(is_robot_bounded(String::from("GGLLGG")), true);
    assert_eq!(is_robot_bounded(String::from("GG")), false);
    assert_eq!(is_robot_bounded(String::from("GL")), true);
}
