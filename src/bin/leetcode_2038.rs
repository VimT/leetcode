//! 如果相邻两个颜色均相同则删除当前颜色

pub fn winner_of_game(colors: String) -> bool {
    let c = colors.as_bytes();
    let mut last = c[0];
    let mut last_cnt = 1;
    let len = c.len();
    let mut a = 0;
    let mut b = 0;
    for i in 1..len {
        if c[i] == last {
            last_cnt += 1;
        } else {
            if last == b'A' {
                a += (last_cnt - 2).max(0);
            } else {
                b += (last_cnt - 2).max(0);
            }

            last = c[i];
            last_cnt = 1;
        }
    }
    if last == b'A' {
        a += (last_cnt - 2).max(0);
    } else {
        b += (last_cnt - 2).max(0);
    }

    // println!("{}, {}", a, b);
    a > b
}

fn main() {
    assert!(!winner_of_game("B".into()));
    assert!(winner_of_game("AAAAABBB".into()));
    assert!(winner_of_game("AAA".into()));
    assert!(!winner_of_game("AA".into()));
    assert!(winner_of_game("AAAABBBAA".into()));
    assert!(!winner_of_game("ABBBBBBBAAA".into()));
}
