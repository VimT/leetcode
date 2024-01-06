//! 回文串重新排列查询

pub fn can_make_palindrome_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let s = s.as_bytes();
    let t: Vec<u8> = s[s.len() / 2..].iter().copied().rev().collect();
    let len = t.len();
    let mut sp = vec![[0; 26]; len + 1];
    let mut tp = vec![[0; 26]; len + 1];
    for i in 0..len {
        sp[i + 1] = sp[i].clone();
        tp[i + 1] = tp[i].clone();
        sp[i + 1][s[i] as usize - b'a' as usize] += 1;
        tp[i + 1][t[i] as usize - b'a' as usize] += 1;
    }
    let mut ne = vec![0; len + 1];
    for i in 0..len {
        ne[i + 1] = ne[i] + (s[i] != t[i]) as i32;
    }
    fn count(a: &Vec<[i32; 26]>, l: usize, r: usize) -> [i32; 26] {
        let mut result = [0; 26];
        for i in 0..26 { result[i] = a[r + 1][i] - a[l][i]; }
        result
    }
    fn sub(a: &[i32; 26], b: &[i32; 26]) -> Option<[i32; 26]> {
        let mut result = [0; 26];
        for i in 0..26 {
            result[i] = a[i] - b[i];
            if result[i] < 0 { return None; }
        }
        Some(result)
    }
    let check = |l1: usize, r1: usize, l2: usize, r2: usize, sp, tp| -> bool {
        assert!(l1 <= l2);
        if ne[l1] > 0 || ne[len] - ne[r1.max(r2) + 1] > 0 { return false; }
        if r1 >= r2 { // 区间包含，检查 [l1, r1] 是否一致
            return count(sp, l1, r1) == count(tp, l1, r1);
        }
        if r1 < l2 { // 区间不相交，检查 [l1, r1] 和 [l2, r2] 是否一致
            return ne[l2] - ne[r1 + 1] == 0 && count(sp, l1, r1) == count(tp, l1, r1) && count(sp, l2, r2) == count(tp, l2, r2);
        }
        // 区间相交
        if let (Some(s1), Some(s2)) = (sub(&count(sp, l1, r1), &count(tp, l1, l2 - 1)), sub(&count(tp, l2, r2), &count(sp, r1 + 1, r2))) {
            return s1 == s2;
        }
        false
    };


    queries.into_iter().map(|query| {
        let (l1, r1, r2, l2) = (query[0] as usize, query[1] as usize, len * 2 - 1 - query[2] as usize, len * 2 - 1 - query[3] as usize);
        return if l1 <= l2 { check(l1, r1, l2, r2, &sp, &tp) } else { check(l2, r2, l1, r1, &tp, &sp) };
    }).collect()
}

fn main() {
    fn test(func: fn(s: String, queries: Vec<Vec<i32>>) -> Vec<bool>) {
        assert_eq!(func(String::from("acbcab"), vec![vec![1, 2, 4, 5]]), vec![true]);
        assert_eq!(func(String::from("abcabc"), vec![vec![1, 1, 3, 5], vec![0, 2, 5, 5]]), vec![true, true]);
        assert_eq!(func(String::from("abbcdecbba"), vec![vec![0, 2, 7, 9]]), vec![false]);
    }
    test(can_make_palindrome_queries);
}
