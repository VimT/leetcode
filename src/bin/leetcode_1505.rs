//! 最多 K 次交换相邻数位后得到的最小整数

use std::ptr::copy;

/// 直接暴力 （1s 过了）
pub fn min_integer(mut num_str: String, mut k: i32) -> String {
    let mut s = unsafe { num_str.as_bytes_mut() };
    while !s.is_empty() && k > 0 {
        for ch in b'0'..=b'9' {
            if let Some(i) = s.iter().position(|&x| x == ch) {
                if i > k as usize { continue; }
                unsafe { copy(s.as_ptr(), s[1..].as_mut_ptr(), i) };
                // for j in (1..=i).rev() { s[j] = s[j - 1]; } // 用这个会超时
                s[0] = ch;
                k -= i as i32;
                s = &mut s[1..];
                break;
            }
        }
    }
    num_str
}

/// 使用树状数组维护一个要移动的数前面已经移动过的次数
pub fn min_integer2(num_str: String, k: i32) -> String {
    struct BIT {
        tree: Vec<i32>,
    }
    impl BIT {
        fn update(&mut self, mut x: usize) {
            while x < self.tree.len() {
                self.tree[x] += 1;
                x += lowbit(x);
            }
        }
        fn query(&mut self, mut x: usize) -> i32 {
            let mut result = 0;
            while x > 0 {
                result += self.tree[x];
                x -= lowbit(x);
            }
            result
        }
    }
    fn lowbit(x: usize) -> usize {
        let x = x as i32;
        (x & -x) as usize
    }

    let mut k = k as usize;
    let len = num_str.len();
    let mut num = num_str.into_bytes();
    let mut bit = BIT { tree: vec![0; len + 1] };
    let mut pos = vec![vec![]; 10];
    for (i, &ch) in num.iter().enumerate() {
        pos[(ch - b'0') as usize].push(i);
    }
    for v in &mut pos { v.reverse(); }
    let mut result = Vec::with_capacity(len);
    while k > 0 {
        let mut dd = false;
        for j in 0..10 {
            if !pos[j].is_empty() {
                // 第i个位置移动到前面所需要的步数 = i - 在i之前已经用过的元素总数
                let will_move = *pos[j].last().unwrap();
                let used = bit.query(will_move) as usize;
                let dist = will_move - used;
                if dist <= k {
                    bit.update(pos[j].pop().unwrap() + 1);
                    result.push(j as u8 + b'0');
                    num[will_move] = 0;
                    k -= dist;
                    dd = true;
                    break;
                }
            }
        }
        if !dd { break; }
    }
    for ch in num {
        if ch > 0 { result.push(ch); }
    }
    unsafe { String::from_utf8_unchecked(result) }
}

fn main() {
    fn test(func: fn(num: String, k: i32) -> String) {
        assert_eq!(func(String::from("4321"), 4), String::from("1342"));
        assert_eq!(func(String::from("100"), 1), String::from("010"));
        assert_eq!(func(String::from("36789"), 1000), String::from("36789"));
    }
    test(min_integer);
    test(min_integer2);
}

