//! 统计道路上的碰撞次数

pub fn count_collisions(directions: String) -> i32 {
    let s = directions.as_bytes();
    let len = s.len();
    let mut i = 0;
    while i < len && s[i] == b'L' { i += 1; }
    let mut j = len;
    while j > 0 && s[j - 1] == b'R' { j -= 1; }
    s[i..j].iter().filter(|x| **x != b'S').count() as i32
}

fn main() {
    assert_eq!(count_collisions(String::from("SSRSSRLLRSLLRSRSSRLRRRRLLRRLSSRR")), 20);
    assert_eq!(count_collisions(String::from("RLRSLL")), 5);
    assert_eq!(count_collisions(String::from("LLRR")), 0);
}
