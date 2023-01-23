//! 替换子串得到平衡字符串

pub fn balanced_string(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let target_len = s.len() / 4;
    fn idx(ch: u8) -> usize {
        match ch {
            b'Q' => 0,
            b'W' => 1,
            b'E' => 2,
            b'R' => 3,
            _ => unreachable!()
        }
    }
    let mut cnt = [0i32; 4];
    for &ch in s {
        cnt[idx(ch)] += 1;
    }
    for x in &mut cnt {
        *x = (*x - target_len as i32).max(0)
    }
    if (0..4).all(|x| cnt[x] == 0) { return 0; }
    let mut result = len;
    let mut cur_cnt = [0; 4];
    let mut r = 0;
    for (l, &ch) in s.iter().enumerate() {
        while r < len && (0..4).any(|i| cur_cnt[i] < cnt[i]) {
            cur_cnt[idx(s[r])] += 1;
            r += 1;
        }
        if !(0..4).any(|i| cur_cnt[i] < cnt[i]) {
            result = result.min(r - l);
        }
        cur_cnt[idx(ch)] -= 1;
    }
    result as i32
}

fn main() {
    fn test(func: fn(s: String) -> i32) {
        assert_eq!(func(String::from("QWER")), 0);
        assert_eq!(func(String::from("QQWE")), 1);
        assert_eq!(func(String::from("QQQW")), 2);
    }
    test(balanced_string);
}
