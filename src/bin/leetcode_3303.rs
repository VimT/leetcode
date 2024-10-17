//! 第一个几乎相等子字符串的下标

/// 前后缀分解 + z 函数
pub fn min_starting_index(s: String, pattern: String) -> i32 {
    fn calc_z(s: &[u8]) -> Vec<usize> {
        let len = s.len();
        let mut z = vec![0; len];
        let mut box_l = 0;
        let mut box_r = 0;
        for i in 1..len {
            if i <= box_r {
                z[i] = z[i - box_l].min(box_r - i + 1);
            }
            while i + z[i] < len && s[z[i]] == s[i + z[i]] {
                box_l = i;
                box_r = i + z[i];
                z[i] += 1;
            }
        }
        z
    }

    let pre_z = calc_z(&(pattern.clone() + &s).into_bytes());
    let mut suf_z = calc_z(&(pattern.chars().rev().collect::<String>() + &s.chars().rev().collect::<String>()).into_bytes());
    suf_z.reverse();
    let len = pattern.len();
    for i in len..s.len() + 1 {
        if pre_z[i] + suf_z[i - 1] >= len - 1 {
            return (i - len) as i32;
        }
    }
    -1
}

fn main() {
    fn test(func: fn(s: String, pattern: String) -> i32) {
        assert_eq!(func(String::from("abcdefg"), String::from("bcdffg")), 1);
        assert_eq!(func(String::from("ababbababa"), String::from("bacaba")), 4);
        assert_eq!(func(String::from("abcd"), String::from("dba")), -1);
        assert_eq!(func(String::from("dde"), String::from("d")), 0);
    }
    test(min_starting_index);
}
