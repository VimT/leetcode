//! 至少有 K 个重复字符的最长子串


/// 分治
pub fn longest_substring(s: String, k: i32) -> i32 {
    if s.len() == 0 { return 0; }
    fn inner(s: &[u8], mut start: usize, mut end: usize, k: usize) -> i32 {
        let mut counter = [0; 26];
        for &i in &s[start..=end] {
            counter[(i - b'a') as usize] += 1;
        }

        while start <= end && counter[(s[start] - b'a') as usize] < k {
            start += 1;
        }
        while start <= end && counter[(s[end] - b'a') as usize] < k {
            end -= 1;
        }
        if end - start + 1 < k { return 0; }

        let mut partition = start;
        while partition < end && counter[(s[partition] - b'a') as usize] >= k {
            partition += 1;
        }
        if partition == end { return (end - start + 1) as i32; }

        return inner(s, start, partition - 1, k).max(inner(s, partition + 1, end, k));
    }
    return inner(s.as_bytes(), 0, s.len() - 1, k as usize);
}

/// 滑动窗口：限定t=1..26 字母数量，给他一个合适的理由收缩左边界. 妙啊。。
pub fn longest_substring_window(s: String, k: i32) -> i32 {
    let mut ret = 0;
    let s = s.as_bytes();
    let len = s.len();
    for t in 1..=26 {
        let mut l = 0;
        let mut r = 0;
        let mut cnt = [0; 26];
        let mut tot = 0;
        let mut less = 0;
        while r < len {
            let ri = (s[r] - b'a') as usize;
            cnt[ri] += 1;
            if cnt[ri] == 1 {
                tot += 1;
                less += 1;
            }
            if cnt[ri] == k {
                less -= 1;
            }
            while tot > t {
                let li = (s[l] - b'a') as usize;
                cnt[li] -= 1;
                if cnt[li] == k - 1 {
                    less += 1;
                }
                if cnt[li] == 0 {
                    tot -= 1;
                    less -= 1;
                }
                l += 1;
            }
            if less == 0 {
                ret = ret.max(r + 1 - l);
            }
            r += 1;
        }
    }
    ret as i32
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> i32) {
        assert_eq!(func(String::from("aaabb"), 3), 3);
        assert_eq!(func(String::from("ababbc"), 2), 5);
    }
    test(longest_substring);
    test(longest_substring_window);
}
