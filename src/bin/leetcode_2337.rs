//! 移动片段得到字符串

pub fn can_change(start: String, target: String) -> bool {
    let s = start.as_bytes();
    let t = target.as_bytes();
    let ss: Vec<u8> = s.iter().filter(|x| **x != b'_').cloned().collect();
    let tt: Vec<u8> = t.iter().filter(|x| **x != b'_').cloned().collect();
    if ss != tt { return false; }

    let len = s.len();
    let mut j = 0;
    for i in 0..len {
        if s[i] == b'_' { continue; }
        while t[j] == b'_' { j += 1; }
        // 如果是L，则i>j，如果是R，则i<j
        if i != j && (s[i] == b'L') != (i > j) { return false; }
        j += 1;
    }
    true
}

fn main() {
    assert_eq!(can_change(String::from("_L__R__RL"), String::from("L_____RLR")), false);
    assert_eq!(can_change(String::from("_L__R__R_"), String::from("L______RR")), true);
    assert_eq!(can_change(String::from("R_L_"), String::from("__LR")), false);
    assert_eq!(can_change(String::from("_R"), String::from("R_")), false);
}
