//! 数字转换为十六进制数

pub fn to_hex(num: i32) -> String {
    let mut result = vec![];
    if num == 0 { result.push(b'0'); }
    let mut num = num as u32;
    while num != 0 {
        result.push(match num & 0b1111 {
            0b1111 => b'f',
            0b1110 => b'e',
            0b1101 => b'd',
            0b1100 => b'c',
            0b1011 => b'b',
            0b1010 => b'a',
            x => x as u8 + b'0',
        });
        num >>= 4;
    }
    result.reverse();
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    assert_eq!(to_hex(0), String::from("0"));
    assert_eq!(to_hex(26), String::from("1a"));
    assert_eq!(to_hex(-1), String::from("ffffffff"));
}
