//! 每种字符至少取 K 个

pub fn take_characters(s: String, k: i32) -> i32 {
    if k == 0 { return 0; }
    let s = s.as_bytes();
    let len = s.len();
    let mut right = vec![vec![0]; 3];  // f[x] = i，记录这个字母，从右侧第x次出现是第i分钟
    for i in (0..len).rev() {
        if right[(s[i] - b'a') as usize].len() <= k as usize {
            right[(s[i] - b'a') as usize].push(len - i);
        }
    }
    for i in 0..3 {
        if right[i].len() <= k as usize { return -1; }
    }
    let mut cnt = [0; 3];
    let mut result = (0..3).map(|x| right[x][(k - cnt[x]).max(0) as usize] as i32).max().unwrap();  // 全部取右侧
    for i in 0..len {
        cnt[(s[i] - b'a') as usize] += 1;
        result = result.min(i as i32 + 1 + (0..3).map(|x| right[x][(k - cnt[x]).max(0) as usize] as i32).max().unwrap())
    }
    result
}

/// 双指针，O(1)的空间复杂度
pub fn take_characters2(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let len = s.len();

    let mut cnt = [0; 3];
    let mut j = len;  //右指针
    while *cnt.iter().min().unwrap() < k {
        if j == 0 { return -1; }
        cnt[(s[j - 1] - b'a') as usize] += 1;
        j -= 1;
    }
    let mut result = len - j;
    // 左指针遍历
    for (i, &ch) in s.iter().enumerate() {
        cnt[(ch - b'a') as usize] += 1;
        while j < len && cnt[(s[j] - b'a') as usize] > k {
            cnt[(s[j] - b'a') as usize] -= 1;
            j += 1;
        }
        result = result.min(i + 1 + len - j);
        if j == len { break; }
    }
    result as i32
}

/// s+s，然后二分+滑动窗口
pub fn take_characters3(s: String, k: i32) -> i32 {
    let len = s.len();
    let s = s.clone() + &s;
    let s = s.as_bytes();
    let mut left = 0;
    let mut right = len + 1;
    while left < right {
        let mid = (left + right) / 2;
        let mut cnt = [0; 3];
        for i in len - mid..len {
            cnt[(s[i] - b'a') as usize] += 1;
        }
        let mut ok = *cnt.iter().min().unwrap() >= k;
        if !ok {
            for i in len..len + mid {
                cnt[(s[i] - b'a') as usize] += 1;
                cnt[(s[i - mid] - b'a') as usize] -= 1;
                if *cnt.iter().min().unwrap() >= k {
                    ok = true;
                    break;
                }
            }
        }
        if ok {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    if left == len + 1 { -1 } else { left as i32 }
}

fn main() {
    fn test(func: fn(s: String, k: i32) -> i32) {
        assert_eq!(func(String::from("aabaaaacaabc"), 2), 8);
        assert_eq!(func(String::from("cbbac"), 0), 0);
        assert_eq!(func(String::from("cbbac"), 1), 3);
        assert_eq!(func(String::from("a"), 1), -1);
    }
    test(take_characters);
    test(take_characters2);
    test(take_characters3);
}
