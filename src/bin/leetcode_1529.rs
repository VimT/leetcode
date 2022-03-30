//! 灯泡开关 IV

pub fn min_flips(target: String) -> i32 {
    let t = target.bytes().skip_while(|x| *x == b'0').collect::<Vec<u8>>();
    if t.len() == 0 { return 0; }
    let mut ans = 1;
    for i in 1..t.len() {
        if t[i] != t[i - 1] {
            ans += 1;
        }
    }
    ans
}

fn main() {
    assert_eq!(min_flips("101".to_string()), 3);
    assert_eq!(min_flips("000000".to_string()), 0);
    assert_eq!(min_flips("001011101".to_string()), 5);
}