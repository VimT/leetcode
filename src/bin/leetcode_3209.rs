//! 子数组按位与值为 K 的数目

/// 3097，子数组二进制运算模板
pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let len = nums.len();
    let mut ors = vec![]; // (and, index, number) 表示 下标 index 为结尾的子数组的左端点为 and 的数有 number 个
    let mut result = 0;
    for i in 0..len {
        ors.push((i32::MAX, i, 1));
        let mut j = 0;
        for p in 0..ors.len() {
            ors[p].0 &= nums[i];
            if ors[p].0 < k { continue; }
            if ors[p].0 == k {
                result += ors[p].2;
            }
            if ors[j].0 == ors[p].0 {
                if j == p { continue; }
                ors[j].2 += ors[p].2;  // 合并
            } else {
                j += 1;
                ors[j] = ors[p];
            }
        }
        ors.truncate(j + 1);
    }
    result
}

/// 二分查找
/// 每个元素都是其右侧元素的子集，所以从 nums[0] 到 nums[i] 的元素值是非降的
pub fn count_subarrays2(mut nums: Vec<i32>, k: i32) -> i64 {
    let mut result = 0;
    let len = nums.len();
    for i in 0..len {
        let x = nums[i];
        for j in (0..i).rev() {
            if nums[j] & x == nums[j] {  // nums[j] 是 nums[i] 的子集，nums[..j] 是 nums[j] 的子集，所以可以 break
                break;
            }
            nums[j] &= x;
        }
        result += nums[..=i].binary_search_by(|x| x.cmp(&k).then(std::cmp::Ordering::Less)).unwrap_err() -
            nums[..=i].binary_search_by(|x| x.cmp(&k).then(std::cmp::Ordering::Greater)).unwrap_err();
    }
    result as i64
}

/// 三指针。发现二分查找的时候，left和right是单调递增的，可以使用滑动窗口优化
pub fn count_subarrays3(mut nums: Vec<i32>, k: i32) -> i64 {
    let mut left = 0;
    let mut right = 0;
    let len = nums.len();
    let mut result = 0;
    for i in 0..len {
        let x = nums[i];
        for j in (0..i).rev() {
            if nums[j] & x == nums[j] { break; }
            nums[j] &= x;
        }
        while left <= i && nums[left] < k { left += 1; }
        while right <= i && nums[right] <= k { right += 1; }
        result += right - left;
    }
    result as i64
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> i64) {
        assert_eq!(func(vec![1, 1, 1], 1), 6);
        assert_eq!(func(vec![1, 1, 2], 1), 3);
        assert_eq!(func(vec![1, 2, 3], 2), 2);
    }
    test(count_subarrays);
    test(count_subarrays2);
    test(count_subarrays3);
}
