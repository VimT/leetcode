//! 中心对称数

pub fn is_strobogrammatic(num: String) -> bool {
    let s = num.as_bytes();
    let mut rotate_num = [255; 10];
    rotate_num[0] = 0;
    rotate_num[1] = 1;
    rotate_num[6] = 9;
    rotate_num[8] = 8;
    rotate_num[9] = 6;
    let mut cs = vec![];
    for &ch in s.iter().rev() {
        if rotate_num[(ch - b'0') as usize] == 255 { return false; }
        cs.push(rotate_num[(ch - b'0') as usize] + b'0');
    }
    cs == s
}

fn main() {
    fn test(func: fn(num: String) -> bool) {
        assert_eq!(func(String::from("69")), true);
        assert_eq!(func(String::from("88")), true);
        assert_eq!(func(String::from("962")), false);
    }
    test(is_strobogrammatic);
}
