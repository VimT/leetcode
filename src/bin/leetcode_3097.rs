//! 或值至少为 K 的最短子数组 II

/// 滑动窗口
pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let mut bit = [0; 31];
    let mut result = i32::MAX;
    let mut l = 0;
    let mut cur = 0;
    for r in 0..len {
        for i in 0..31 {
            if nums[r] >> i & 1 == 1 {
                bit[i] += 1;
            }
        }
        cur |= nums[r];
        while cur >= k && l <= r {
            result = result.min((r - l + 1) as i32);
            for i in 0..31 {
                if nums[l] >> i & 1 == 1 {
                    bit[i] -= 1;
                    if bit[i] == 0 {
                        cur &= !(1 << i);
                    }
                }
            }
            l += 1;
        }
    }
    if result == i32::MAX { -1 } else { result }
}


/// 类似 2411，通用模板
/// 对于 OR 运算。对于以 i 结尾的子数组，左端点为 [0, i) 的子数组，最多有 30 个不同的数
pub fn minimum_subarray_length2(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let mut ors = vec![]; // (max_or, index) 表示 下标 index 为结尾的子数组的左端点的最大或值
    let mut result = i32::MAX;
    for i in 0..len {
        ors.push((0, i));
        let mut j = 0;
        for p in 0..ors.len() {
            ors[p].0 |= nums[i];
            if ors[p].0 >= k {
                result = result.min((i - ors[p].1 + 1) as i32);
            }
            if ors[j].0 == ors[p].0 {
                ors[j].1 = ors[p].1;  // 原地去重
            } else {
                j += 1;
                ors[j] = ors[p];
            }
        }
        ors.truncate(j + 1);
    }
    if result == i32::MAX { -1 } else { result }
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i32) {
        assert_eq!(func(vec![1, 2, 3], 2), 1);
        assert_eq!(func(vec![2, 1, 8], 10), 3);
        assert_eq!(func(vec![1, 2], 0), 1);
    }
    test(minimum_subarray_length);
    test(minimum_subarray_length2);
}
