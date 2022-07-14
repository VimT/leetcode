//! 列举单词的全部缩写

/// 二进制遍历
pub fn generate_abbreviations(word: String) -> Vec<String> {
    let s = word.as_bytes();
    let len = s.len();
    let mut result = vec![String::new(); 1 << len];
    for i in 0..1 << len {
        let mut tmp = vec![];
        let mut cnt = 0;
        for j in 0..len {
            if i >> (len - 1 - j) & 1 == 1 {
                if cnt > 0 {
                    tmp.extend_from_slice(cnt.to_string().as_bytes());
                    cnt = 0;
                }
                tmp.push(s[j]);
            } else {
                cnt += 1;
            }
        }
        if cnt > 0 {
            tmp.extend_from_slice(cnt.to_string().as_bytes());
        }
        result[i] = unsafe { String::from_utf8_unchecked(tmp) };
    }
    result
}

fn main() {
    fn test(func: fn(word: String) -> Vec<String>) {
        assert_eq!(func(String::from("word")), vec!["4", "3d", "2r1", "2rd", "1o2", "1o1d", "1or1", "1ord", "w3", "w2d", "w1r1", "w1rd", "wo2", "wo1d", "wor1", "word"]);
        assert_eq!(func(String::from("a")), vec!["1", "a"]);
    }
    test(generate_abbreviations);
}
