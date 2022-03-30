//! 从英文中重建数字

pub fn original_digits(s: String) -> String {
    // 'z': [0],
    // 'e': [0, 1, 3, 3, 5, 7, 7, 8, 9],
    // 'r': [0, 3, 4],
    // 'o': [0, 1, 2, 4],
    // 'n': [1, 7, 9, 9],
    // 't': [2, 3, 8],
    // 'w': [2],
    // 'h': [3, 8],
    // 'f': [4, 5],
    // 'u': [4],
    // 'i': [5, 6, 8, 9],
    // 'v': [5, 7],
    // 's': [6, 7],
    // 'x': [6],
    // 'g': [8]
    #[inline]
    fn idx(ch: u8) -> usize { (ch - b'a') as usize }
    let s = s.as_bytes();
    let mut cnt = [0; 26];
    for &ch in s {
        cnt[idx(ch)] += 1;
    }
    let mut result = [0; 10];

    result[0] = cnt[idx(b'z')];
    result[2] = cnt[idx(b'w')];
    result[4] = cnt[idx(b'u')];
    result[6] = cnt[idx(b'x')];
    result[8] = cnt[idx(b'g')];
    result[3] = cnt[idx(b'h')] - result[8];
    result[5] = cnt[idx(b'f')] - result[4];
    result[7] = cnt[idx(b's')] - result[6];
    result[1] = cnt[idx(b'o')] - result[0] - result[2] - result[4];
    result[9] = cnt[idx(b'i')] - result[5] - result[6] - result[8];

    let mut ss = Vec::with_capacity(result.iter().sum::<usize>());
    for i in 0..10 {
        for _ in 0..result[i as usize] {
            ss.push(b'0' + i);
        }
    }
    unsafe { String::from_utf8_unchecked(ss) }
}

fn main() {
    assert_eq!(original_digits(String::from("owoztneoer")), "012");
    assert_eq!(original_digits(String::from("fviefuro")), "45")
}

