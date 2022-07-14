//! 回文排列 II

pub fn generate_palindromes(s: String) -> Vec<String> {
    if s.len() <= 1 {
        return vec![s];
    }
    let s = s.as_bytes();
    let mut cnt = [0; 26];
    for &ch in s {
        cnt[(ch - b'a') as usize] += 1;
    }
    let mut odd = 0;
    let mut odd_char = 255;
    for i in 0..26 {
        if cnt[i] & 1 == 1 {
            odd += 1;
            odd_char = i as u8 + b'a';
        }
    }
    if odd > 1 {
        return vec![];
    }
    let mut ori = vec![];
    for i in 0..26 {
        for _ in 0..cnt[i] / 2 {
            ori.push(i as u8 + b'a');
        }
    }

    // 全排列II
    let len = ori.len();
    let mut ans = vec![ori];
    for position in 0..len - 1 {
        for i in 0..ans.len() {
            ans[i][position..].sort_unstable();
            for j in position + 1..len {
                let mut tmp = ans[i].clone();
                if tmp[j] == tmp[j - 1] { continue; }
                tmp.swap(position, j);
                ans.push(tmp);
            }
        }
    }
    for i in &mut ans {
        let len = i.len();
        if odd == 1 {
            i.push(odd_char);
        }
        for j in (0..len).rev() {
            i.push(i[j]);
        }
    }
    ans.into_iter().map(|x| unsafe { String::from_utf8_unchecked(x) }).collect()
}

fn main() {
    fn test(func: fn(s: String) -> Vec<String>) {
        assert_eq!(func(String::from("aabb")), vec!["abba", "baab"]);
        assert_eq!(func(String::from("abc")).is_empty(), true);
        assert_eq!(func(String::from("a")), vec!["a"]);
    }
    test(generate_palindromes);
}
