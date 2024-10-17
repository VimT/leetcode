//! 元音辅音字符串计数 I

pub fn count_of_substrings(word: String, k: i32) -> i32 {
    let s = word.as_bytes();
    let len = s.len();
    let mut result = 0;
    fn idx(ch: u8) -> usize {
        match ch {
            b'a' => 0,
            b'e' => 1,
            b'i' => 2,
            b'o' => 3,
            b'u' => 4,
            _ => 5,
        }
    }
    let mut l1 = 0;
    let mut l2 = 0;
    let mut cnt1 = [0; 6];
    let mut cnt2 = [0; 6];
    for r in 0..len {
        cnt1[idx(s[r])] += 1;
        cnt2[idx(s[r])] += 1;
        while l1 < r && cnt1[5] > k {
            cnt1[idx(s[l1])] -= 1;
            l1 += 1;
        }
        while l2 < r && (cnt2[5] > k || (idx(s[l2]) < 5 && cnt2[idx(s[l2])] > 1)) {
            cnt2[idx(s[l2])] -= 1;
            l2 += 1;
        }
        if (0..5).all(|i| cnt1[i] > 0) && cnt1[5] == k {
            result += l2 + 1 - l1;
        }
    }
    result as i32
}

fn main() {
    fn test(func: fn(word: String, k: i32) -> i32) {
        assert_eq!(func(String::from("aeioqq"), 1), 0);
        assert_eq!(func(String::from("aeiou"), 0), 1);
        assert_eq!(func(String::from("ieaouqqieaouqq"), 1), 3);
    }
    test(count_of_substrings);
}
