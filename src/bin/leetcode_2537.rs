//! 统计好子数组的数目

use std::collections::HashMap;

pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
    let len = nums.len();
    let mut cur = 0;
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let mut j = 0;
    let mut result = 0;
    // i 开始的好子数组（枚举左端点）
    for i in 0..len {
        while j < len && cur < k {
            let jcnt = cnt.entry(nums[j]).or_default();
            cur += *jcnt;
            *jcnt += 1;
            j += 1;
        }
        if cur >= k {
            result += len - j + 1;
        }
        if j == len && cur < k { break; }
        let icnt = cnt.get_mut(&nums[i]).unwrap();
        *icnt -= 1;
        cur -= *icnt;
    }
    result as i64
}


/// 枚举右端点写法，更简洁（不需要一些判断）
pub fn count_good2(nums: Vec<i32>, k: i32) -> i64 {
    let len = nums.len();
    let mut cur = 0;
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let mut left = 0;
    let mut result = 0;
    for i in 0..len {
        let rcnt = cnt.entry(nums[i]).or_default();
        cur += *rcnt;
        *rcnt += 1;
        result += left;  // 左边的一定可以
        while cur >= k {
            result += 1;
            let lcnt = cnt.get_mut(&nums[left]).unwrap();
            *lcnt -= 1;
            cur -= *lcnt;
            left += 1;
        }
    }
    result as i64
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i64) {
        assert_eq!(func(vec![1, 1, 1, 1, 1], 10), 1);
        assert_eq!(func(vec![3, 1, 4, 3, 2, 2, 4], 2), 4);
    }
    test(count_good);
    test(count_good2);
}
