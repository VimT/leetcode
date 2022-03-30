//! 字母板上的路径

pub fn alphabet_board_path(target: String) -> String {
    let board = [b"abcde", b"fghij", b"klmno", b"pqrst", b"uvwxy"];
    let mut ch_pos = vec![(0, 0); 26];
    for i in 0..5 {
        for j in 0..5 {
            ch_pos[(board[i][j] - b'a') as usize] = (i, j);
        }
    }
    ch_pos[25] = (5, 0);
    let mut cur = (0, 0);
    let mut result = vec![];
    for &ch in target.as_bytes() {
        let mut t = ch_pos[(ch - b'a') as usize];
        if t == cur {
            result.push(b'!');
            continue;
        }
        if ch == b'z' {
            t = ch_pos[(b'u' - b'a') as usize];
        }
        if t.0 > cur.0 {
            for _ in 0..t.0 - cur.0 {
                result.push(b'D');
            }
        } else {
            for _ in 0..cur.0 - t.0 {
                result.push(b'U');
            }
        }
        if t.1 > cur.1 {
            for _ in 0..t.1 - cur.1 {
                result.push(b'R');
            }
        } else {
            for _ in 0..cur.1 - t.1 {
                result.push(b'L');
            }
        }
        if ch == b'z' {
            result.push(b'D');
            t = ch_pos[25];
        }
        result.push(b'!');
        cur = t;
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    fn test(func: fn(target: String) -> String) {
        assert_eq!(func(String::from("zdz")), String::from("DDDDD!UUUUURRR!DDDDLLLD!"));
        assert_eq!(func(String::from("leet")), String::from("DDR!UURRR!!DDD!"));
        assert_eq!(func(String::from("code")), String::from("RR!DDRR!UUL!R!"));
    }
    test(alphabet_board_path);
}
