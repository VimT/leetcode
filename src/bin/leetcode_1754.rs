//! 构造字典序最大的合并字符串

pub fn largest_merge(word1: String, word2: String) -> String {
    let w1 = word1.as_bytes();
    let w2 = word2.as_bytes();
    let mut result = Vec::with_capacity(w1.len() + w2.len());
    let mut i1 = 0;
    let mut i2 = 0;
    let mut j1 = 0;
    let mut j2 = 0;
    while i1 < w1.len() || i2 < w2.len() {
        //找出递增序列, 或者每次只append一个字符
        if i1 == j1 && j1 < w1.len() {
            j1 += 1;
            while j1 < w1.len() && w1[j1] >= w1[j1 - 1] { j1 += 1; }
        }
        if i2 == j2 && j2 < w2.len() {
            j2 += 1;
            while j2 < w2.len() && w2[j2] >= w2[j2 - 1] { j2 += 1; }
        }
        if w1[i1..] > w2[i2..] {
            unsafe { println!("word1[{}..{}]: {}", i1, j1, String::from_utf8_unchecked(w1[i1..j1].to_vec())); }
            result.extend(&w1[i1..j1]);
            i1 = j1;
        } else {
            unsafe { println!("word2[{}..{}]: {}", i2, j2, String::from_utf8_unchecked(w2[i2..j2].to_vec())); }
            result.extend(&w2[i2..j2]);
            i2 = j2;
        }
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    assert_eq!(String::from("guguuuuuuuuuuuuuuguguuuuguugguggggggguuggguuggggggg"), largest_merge(String::from("guguuuuuuuuuuuuuuguguuuuguug"), String::from("gguggggggguuggguugggggg")));
    assert_eq!(String::from("jvjjjjjjvjjvjvjjjvjvjjjjjjvjjjjjjvjjjjvjjjj"), largest_merge(String::from("jvjjjjjjvjjvjvjjjvjvjjjj"), String::from("jjjjjjvjjjjjjvjjjjv")));
    assert_eq!(String::from("abdcabcabcaba"), largest_merge(String::from("abcabc"), String::from("abdcaba")));
    assert_eq!(String::from("uuuurruuuruuuuuuuuruuuuurrrurrrrrrrruurrrurrrurrrrruu"), largest_merge(String::from("uuurruuuruuuuuuuuruuuuu"), String::from("urrrurrrrrrrruurrrurrrurrrrruu")));
    assert_eq!(String::from("cbcabaaaaa"), largest_merge(String::from("cabaa"), String::from("bcaaa")));
}
