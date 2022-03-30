pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }
    let mut s1c = [0; 26];
    for i in s1.as_bytes() {
        s1c[(*i - b'a') as usize] += 1;
    }
    let mut c = [0; 26];
    let mut left = 0;
    let mut right = s1.len();
    let s2 = s2.as_bytes();
    let len = s2.len();
    for i in 0..s1.len() {
        c[(s2[i] - b'a') as usize] += 1;
    }
    while right <= len {
        // too many compare times, can use diff var to record how many diff for every char
        if c == s1c {
            return true;
        }
        if right == len { break; }
        c[(s2[right] - b'a') as usize] += 1;
        right += 1;
        c[(s2[left] - b'a') as usize] -= 1;
        left += 1;
    }
    false
}

pub fn check_inclusion_best(s1: String, s2: String) -> bool {
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    let n = s1.len();
    let m = s2.len();
    if n > m { return false; }
    let mut cnt = [0; 26];
    for i in 0..n {
        cnt[(s1[i] - b'a') as usize] -= 1;
    }
    let mut left = 0;
    for right in 0..m {
        let x = (s2[right] - b'a') as usize;
        cnt[x] += 1;
        while cnt[x] > 0 {
            cnt[(s2[left] - b'a') as usize] -= 1;
            left += 1;
        }
        if right + 1 == n + left {
            return true;
        }
    }
    false
}

fn main() {
    assert!(check_inclusion(String::from("adc"), String::from("dcda")));
    assert!(check_inclusion(String::from("ab"), String::from("eidbaooo")));
    assert!(!check_inclusion(String::from("ab"), String::from("eidboaoo")));

    assert!(check_inclusion_best(String::from("adc"), String::from("dcda")));
    assert!(check_inclusion_best(String::from("ab"), String::from("eidbaooo")));
    assert!(!check_inclusion_best(String::from("ab"), String::from("eidboaoo")));
}
