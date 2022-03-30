//! 最多的不重叠子字符串

use leetcode::svec;

pub fn max_num_of_substrings(s: String) -> Vec<String> {
    let s = s.as_bytes();
    let len = s.len();
    let mut char_min = [len; 26];
    let mut char_max = [0; 26];
    for i in 0..len {
        char_min[(s[i] - b'a') as usize] = char_min[(s[i] - b'a') as usize].min(i);
        char_max[(s[i] - b'a') as usize] = i;
    }
    fn find(s: &[u8], char_min: &[usize; 26], char_max: &[usize; 26], start: usize) -> usize {
        let mut end = char_max[(s[start] - b'a') as usize];
        let mut i = start + 1;
        while i < end {
            if char_min[(s[i] - b'a') as usize] < start {
                return s.len();
            }
            end = end.max(char_max[(s[i] - b'a') as usize]);
            i += 1;
        }
        end
    }
    let mut right_max = len;
    let mut result = vec![String::new()];
    for i in 0..len {
        if char_min[(s[i] - b'a') as usize] == i {
            let right = find(s, &char_min, &char_max, i);
            if right == len {
                continue;
            }
            if i > right_max {
                right_max = right;
                result.push(String::new());
            }
            right_max = right_max.min(right);
            *result.last_mut().unwrap() = unsafe { String::from_utf8_unchecked(s[i..=right].to_vec()) };
        }
    }

    result
}

fn main() {
    assert_eq!(max_num_of_substrings("bbeadcxede".into()), svec!["a", "c", "x", "bb"]);
    assert_eq!(max_num_of_substrings("abab".into()), svec!["abab"]);
    assert_eq!(max_num_of_substrings("adefaddaccc".into()), svec!["e", "f", "ccc"]);
    assert_eq!(max_num_of_substrings("abbaccd".into()), svec!["bb", "cc", "d"]);
}