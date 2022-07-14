//! 解密消息

pub fn decode_message(key: String, mut message: String) -> String {
    let mut map = [0; 26];
    let mut cur_ch = b'a';
    for &ch in key.as_bytes() {
        if ch.is_ascii_lowercase() && map[(ch - b'a') as usize] == 0 {
            map[(ch - b'a') as usize] = cur_ch;
            cur_ch += 1;
        }
    }
    let m = unsafe { message.as_bytes_mut() };
    for ch in m {
        if ch.is_ascii_lowercase() {
            *ch = map[(*ch - b'a') as usize];
        }
    }
    message
}


fn main() {
    assert_eq!(decode_message(String::from("the quick brown fox jumps over the lazy dog"), String::from("vkbs bs t suepuv")), "this is a secret");
    assert_eq!(decode_message(String::from("eljuxhpwnyrdgtqkviszcfmabo"), String::from("zwx hnfx lqantp mnoeius ycgk vcnjrdb")), "the five boxing wizards jump quickly");
}
