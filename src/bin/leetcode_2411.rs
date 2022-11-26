//! 按位或最大的最小子数组长度

/// 双指针
pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut target = vec![0; len];
    let mut cur = 0;
    for i in (0..len).rev() {
        cur |= nums[i];
        target[i] = cur;
    }
    let mut cb = [0; 32];
    let mut result = vec![0; len];
    let mut right = 0;
    cur = 0;
    for i in 0..len {
        while cur != target[i] {
            let mut num = nums[right];
            cur |= num;
            let mut j = 0;
            while num > 0 {
                cb[j] += num & 1;
                num >>= 1;
                j += 1;
            }
            right += 1;
        }
        result[i] = ((right - i) as i32).max(1);
        let mut num = nums[i];
        let mut j = 0;
        while num > 0 {
            cb[j] -= num & 1;
            if cb[j] == 0 {
                cur &= !(1 << j);
            }
            num >>= 1;
            j += 1;
        }
    }
    result
}

/// p记录每一位最后一次出现的位置
pub fn smallest_subarrays2(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut p = [0; 30];
    let mut result = vec![0; len];
    for i in (0..len).rev() {
        let mut t = i;
        for j in 0..30 {
            if nums[i] >> j & 1 == 1 { p[j] = i; } else if p[j] != len { t = t.max(p[j]); }
        }
        result[i] = (t - i + 1) as i32;
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> Vec<i32>) {
        assert_eq!(func(vec![1, 0, 2, 1, 3]), vec![3, 3, 2, 2, 1]);
        assert_eq!(func(vec![1, 2]), vec![2, 1]);
    }
    test(smallest_subarrays);
    test(smallest_subarrays2);
}
