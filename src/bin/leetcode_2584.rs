//! 分割数组使乘积互质


use std::collections::HashMap;

/// 对每个质因子的 出现区间[a,b]，求第一次区间不覆盖的位置。
pub fn find_valid_split(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut pos: HashMap<i32, (usize, usize)> = HashMap::with_capacity(1e6 as usize);
    // 分解质因数
    for (i, mut x) in nums.into_iter().enumerate() {
        let mut d = 2;
        while d * d <= x {
            if x % d == 0 {
                x /= d;
                while x % d == 0 { x /= d; }
                pos.entry(d).and_modify(|x| x.1 = i).or_insert((i, i));
            }
            d += 1;
        }
        if x > 1 { pos.entry(x).and_modify(|v| v.1 = i).or_insert((i, i)); }
    }
    // 差分找区间不覆盖的位置
    let mut diff = vec![0; len + 1];
    for (_, (s, e)) in pos {
        diff[s] += 1;
        diff[e] -= 1;
    }
    let mut sum = 0;
    for (i, &v) in diff[..len - 1].iter().enumerate() {
        sum += v;
        if sum == 0 {
            return i as i32;
        }
    }
    -1
}

/// 优化：
/// 1. 不用存储每个质数出现的位置，存储每个数字的质数的覆盖位置，这样天然有序
/// 2. 对有序的区间做区间合并
pub fn find_valid_split2(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut left: HashMap<i32, usize> = HashMap::new(); // 每个质因子第一次出现的位置
    let mut right = vec![0; len];  // 每一组数的质因子的最右边的的数
    let mut insert = |p: i32, i: usize| {
        if let Some(&j) = left.get(&p) {
            right[j] = i;
        } else {
            left.insert(p, i);
        }
    };
    // 分解质因数
    for (i, mut x) in nums.into_iter().enumerate() {
        let mut d = 2;
        while d * d <= x {
            if x % d == 0 {
                x /= d;
                while x % d == 0 { x /= d; }
                insert(d, i);
            }
            d += 1;
        }
        if x > 1 { insert(x, i); }
    }
    let mut max_t = 0;
    for (s, t) in right.into_iter().enumerate() {
        if s > max_t {
            return max_t as i32;
        }
        max_t = max_t.max(t);
    }
    -1
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![557663, 280817, 472963, 156253, 273349, 884803, 756869, 763183, 557663, 964357, 821411, 887849, 891133, 453379, 464279, 574373, 852749, 15031, 156253, 360169, 526159, 410203, 6101, 954851, 860599, 802573, 971693, 279173, 134243, 187367, 896953, 665011, 277747, 439441, 225683, 555143, 496303, 290317, 652033, 713311, 230107, 770047, 308323, 319607, 772907, 627217, 119311, 922463, 119311, 641131, 922463, 404773, 728417, 948281, 612373, 857707, 990589, 12739, 9127, 857963, 53113, 956003, 363397, 768613, 47981, 466027, 981569, 41597, 87149, 55021, 600883, 111953, 119083, 471871, 125641, 922463, 562577, 269069, 806999, 981073, 857707, 831587, 149351, 996461, 432457, 10903, 921091, 119083, 72671, 843289, 567323, 317003, 802129, 612373]), 18);
        assert_eq!(func(vec![4, 7, 8, 15, 3, 5]), 2);
        assert_eq!(func(vec![4, 7, 15, 8, 3, 5]), -1);
    }
    test(find_valid_split);
    test(find_valid_split2);
}
