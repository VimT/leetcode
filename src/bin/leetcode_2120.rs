//! 执行所有后缀指令

pub fn execute_instructions(n: i32, start_pos: Vec<i32>, s: String) -> Vec<i32> {
    let s = s.as_bytes();
    let len = s.len();
    let mut result = vec![0; len];
    for i in 0..len {
        let mut x = start_pos[0];
        let mut y = start_pos[1];
        let mut cnt = 0;
        for j in i..len {
            match s[j] {
                b'L' => y -= 1,
                b'R' => y += 1,
                b'D' => x += 1,
                b'U' => x -= 1,
                _ => panic!(),
            }
            if !(x >= 0 && x < n && y >= 0 && y < n) {
                break;
            }
            cnt += 1;
        }
        result[i] = cnt;
    }

    result
}

fn main() {
    assert_eq!(execute_instructions(3, vec![0, 1], String::from("RRDDLU")), vec![1, 5, 4, 3, 1, 0]);
    assert_eq!(execute_instructions(2, vec![1, 1], String::from("LURD")), vec![4, 1, 0, 0]);
    assert_eq!(execute_instructions(1, vec![0, 0], String::from("LRUD")), vec![0, 0, 0, 0]);
}
