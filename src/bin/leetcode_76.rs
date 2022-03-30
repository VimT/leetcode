//! 最小覆盖子串

use std::collections::HashMap;

/// 滑动窗口
pub fn min_window(s: String, t: String) -> String {
    if s.len() == 0 || t.len() == 0 { return "".to_string(); }
    let mut map = HashMap::new();
    for i in t.bytes() {
        *map.entry(i).or_insert(0) += 1;
    }
    let required = map.len();
    let mut filter = vec![];
    for (i, c) in s.bytes().enumerate() {
        if map.contains_key(&c) {
            filter.push((i, c));
        }
    }
    let mut left = 0;
    let mut right = 0;
    let mut formed = 0;
    let mut window_counts = HashMap::new();
    let mut ans: (i32, usize, usize) = (-1, 0, 0);
    // 滑动窗口模板
    while right < filter.len() {
        // window.add(right)
        let c = filter[right].1;
        *window_counts.entry(c).or_insert(0) += 1;
        if map.contains_key(&c) && window_counts.get(&c).unwrap() == map.get(&c).unwrap() {
            formed += 1;
        }

        // while window need shrink, left += 1, window.remove(left)
        while formed == required {
            let c = filter[left].1;
            let end = filter[right].0;
            let start = filter[left].0;
            let distance = (end - start + 1) as i32;
            if distance < ans.0 || ans.0 == -1 {
                ans.1 = start;
                ans.2 = end;
                ans.0 = distance;
            }

            window_counts.entry(c).and_modify(|x| *x -= 1);
            if map.contains_key(&c) && window_counts.get(&c).unwrap() < map.get(&c).unwrap() {
                formed -= 1;
            }
            left += 1;
        }
        // right += 1
        right += 1;
    }
    return if ans.0 == -1 { "".to_string() } else { s[ans.1..ans.2 + 1].to_string() };
}

pub fn min_window_best(s: String, t: String) -> String {
    if s.is_empty() || t.is_empty() || t.len() > s.len() {
        return "".to_string();
    }
    let mut table = [0usize; 256];
    let t = t.into_bytes();
    for c in t.iter() {
        table[*c as usize] += 1;
    }
    let mut count = 0usize;
    let mut window = [0usize; 256];
    let mut right = 0usize;
    let s = s.into_bytes();
    for (i, c) in s.iter().enumerate() {
        let c = *c as usize;
        if table[c] == 0 { continue; }
        window[c] += 1;
        if window[c] <= table[c] {
            count += 1;
            if count == t.len() {
                right = i;
                break;
            }
        }
    }
    if count != t.len() {
        return "".to_string();
    }
    let mut left = 0usize;
    let mut n = right + 1;
    for i in 0..s.len() - t.len() {
        let c1 = s[i] as usize;
        if window[c1] > 0 {
            window[c1] -= 1;
        }
        if window[c1] < table[c1] {
            let mut r = None;
            for j in right + 1..s.len() {
                let c2 = s[j] as usize;
                if table[c2] == 0 {
                    continue;
                }
                window[c2] += 1;
                if c2 == c1 {
                    r = Some(j);
                    break;
                }
            }
            if let Some(r) = r {
                right = r;
            } else {
                break;
            }
        }
        let m = right - i;
        if m < n {
            left = i + 1;
            n = m;
        }
    }
    String::from_utf8(s[left..left + n].to_vec()).unwrap()
}


fn main() {
    fn test(func: fn(s: String, t: String) -> String) {
        assert_eq!(func(String::from("ADOBECODEBANC"), String::from("ABC")), String::from("BANC"));
        assert_eq!(func(String::from("a"), String::from("a")), String::from("a"));
        assert_eq!(func(String::from("a"), String::from("aa")), String::from(""));
    }
    test(min_window);
    test(min_window_best);
}
