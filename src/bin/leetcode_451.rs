//! 根据字符出现频率排序

pub fn frequency_sort(s: String) -> String {
    let mut s = s.as_bytes().to_vec();
    let mut m = [0; 128];
    for &ch in &s {
        m[ch as usize] += 1;
    }
    s.sort_unstable_by_key(|&x| (-m[x as usize], x));
    unsafe { String::from_utf8_unchecked(s) }
}

fn main() {
    assert_eq!(frequency_sort(String::from("tree")), String::from("eert"));
    assert_eq!(frequency_sort(String::from("cccaaa")), String::from("aaaccc"));
    assert_eq!(frequency_sort(String::from("Aabb")), String::from("bbAa"));
}
