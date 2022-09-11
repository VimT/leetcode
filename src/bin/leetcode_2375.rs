//! 根据模式串构造最小数字

pub fn smallest_number(pattern: String) -> String {
    fn backtrack(pattern: &[u8], i: usize, cur: &mut Vec<u8>, ans: &mut Vec<u8>, seen: &mut Vec<bool>) {
        if i == pattern.len() {
            *ans = cur.clone();
            return
        }
        let (start, end) = if pattern[i] == b'I' {
            (cur[i] + 1, 9)
        } else {
            (1, cur[i] - 1)
        };
        for num in start..=end {
            if !seen[num as usize] {
                cur.push(num);
                seen[num as usize] = true;
                backtrack(pattern, i + 1, cur, ans, seen);
                if !ans.is_empty() { return; }
                seen[num as usize] = false;
                cur.pop();
            }
        }
    }
    let mut ans = vec![];
    let mut seen = vec![false; 10];
    let mut cur = vec![];
    for i in 1..=9 {
        cur.push(i);
        seen[i as usize] = true;
        backtrack(pattern.as_bytes(), 0, &mut cur, &mut ans, &mut seen);
        if !ans.is_empty() { break; }
        cur.pop();
        seen[i as usize] = false;
    }
    ans.into_iter().map(|x| (x + b'0') as char).collect()
}

fn main() {
    assert_eq!(smallest_number(String::from("IIIDIDDD")), "123549876");
    assert_eq!(smallest_number(String::from("DDD")), "4321");
}
