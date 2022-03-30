//! 划分字母区间

pub fn partition_labels(s: String) -> Vec<i32> {
    let mut mm = vec![(usize::MAX, 0); 26];
    let s = s.as_bytes();
    let len = s.len();
    for i in 0..len {
        let ch = (s[i] - b'a') as usize;
        mm[ch].0 = mm[ch].0.min(i);
        mm[ch].1 = mm[ch].1.max(i);
    }
    let mut result = vec![];
    let mut start = 0;
    while start < len {
        let mut end = mm[(s[start] - b'a') as usize].1;
        let mut i = start;
        while i < end {
            end = end.max(mm[(s[i] - b'a') as usize].1);
            i += 1;
        }
        result.push((end + 1 - start) as i32);
        start = end + 1;
    }
    result
}

fn main() {
    assert_eq!(partition_labels(String::from("ababcbacadefegdehijhklij")), vec![9, 7, 8]);
    assert_eq!(partition_labels(String::from("eccbbbbdec")), vec![10]);
}
