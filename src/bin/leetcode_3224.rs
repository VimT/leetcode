//! 使差值相等的最少数组改动次数

use std::collections::HashMap;

/// 枚举差值出现次数最多的
pub fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
    let mut cnt: HashMap<u32, i32> = HashMap::new();
    let len = nums.len();
    for i in 0..len / 2 {
        *cnt.entry(nums[i].abs_diff(nums[len - 1 - i])).or_default() += 1;
    }
    let mut cnt = cnt.into_iter().collect::<Vec<_>>();
    cnt.sort_by_key(|x| -x.1);

    let mut result = len as i32;
    // 差值为 x 的时候，有多少个是可以不用改动的
    for (x, ok) in cnt {
        if (len as i32 / 2 - ok) > result {
            break;
        }
        let mut need = 0;
        for i in 0..len / 2 {
            if nums[i].abs_diff(nums[len - 1 - i]) != x {
                let x = x as i32;
                if (nums[i] - x < 0 && nums[i] + x > k) && (nums[len - i - 1] - x < 0 && nums[len - i - 1] + x > k) {
                    need += 2;
                } else {
                    need += 1;
                }
            }
        }
        result = result.min(need);
    }
    result
}

/// 枚举 x
pub fn min_changes2(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut cnt = vec![0; k + 1];
    let mut cnt2 = vec![0; k + 1];
    let len = nums.len();
    for i in 0..len / 2 {
        let (mut p, mut q) = (nums[i], nums[len - 1 - i]);
        if p > q {
            std::mem::swap(&mut p, &mut q);
        }
        cnt[(q - p) as usize] += 1;
        // p < q：
        // 如果改 p：把 p 改成 0 可以让差值最大
        // 如果改 q：把 q 改成 k 可以让差值最大
        // 当 p.max(k-p) >= x 的时候，可以只改一个数，否则要改两个
        cnt2[q.max(k as i32 - p) as usize] += 1;
    }
    let mut result = len as i32;
    let mut sum2 = 0;  // 统计有多少对 (p, q) 需要改
    for (c, c2) in cnt.into_iter().zip(cnt2) {
        // 其他 n/2 - c 个数对至少要改一个数，有额外的 sum2 个数对需要改两个数
        result = result.min(len as i32 / 2 - c + sum2);
        sum2 += c2;
    }
    result
}


fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![7, 0, 0, 4, 3, 6, 5, 5], 7), 3);
        assert_eq!(func(vec![1, 0, 1, 2, 4, 3], 4), 2);
        assert_eq!(func(vec![0, 1, 2, 3, 3, 6, 5, 4], 6), 2);
    }
    test(min_changes);
    test(min_changes2);
}
