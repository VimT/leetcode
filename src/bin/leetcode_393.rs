//! UTF-8 编码验证

pub fn valid_utf8(data: Vec<i32>) -> bool {
    let mut i = 0;
    let len = data.len();
    while i < len {
        let char_len = (data[i] as u8).leading_ones();
        match char_len {
            0 => i += 1,
            2 | 3 | 4 => {
                for j in i + 1..i + char_len as usize {
                    if j >= len || data[j] >> 6 != 0b10 {
                        return false;
                    }
                }
                i += char_len as usize;
            }
            _ => { return false; }
        }
    }
    i == len
}

fn main() {
    assert_eq!(valid_utf8(vec![250, 145, 145, 145, 145]), false);
    assert_eq!(valid_utf8(vec![145]), false);
    assert_eq!(valid_utf8(vec![197, 130, 1]), true);
    assert_eq!(valid_utf8(vec![235, 140, 4]), false);
}
