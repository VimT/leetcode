//! 统计上升四元组

/// 枚举中间两个数
/// j左边<nums[k] * k右边>nums[j]的
pub fn count_quadruplets(nums: Vec<i32>) -> i64 {
    let len = nums.len();
    let mut result = 0;
    let mut right_great = vec![vec![0i64; len + 1]; len];
    for k in (2..len - 1).rev() {
        right_great[k] = right_great[k + 1].clone();
        for x in 1..nums[k + 1] as usize {
            right_great[k][x] += 1;
        }
    }
    let mut left_less = vec![0i64; len + 1];
    for j in 1..len - 2 {
        for x in (nums[j - 1] + 1) as usize..=len {
            left_less[x] += 1;
        }
        for k in j + 1..len - 1 {
            if nums[j] > nums[k] {
                result += left_less[nums[k] as usize] * right_great[k][nums[j] as usize]
            }
        }
    }
    result
}

/// 统计 132 这样顺序的三元组数目
/// cnt[i] 表示 nums[i]是3 的132三元组数目
/// 妙啊！
pub fn count_quadruplets2(nums: Vec<i32>) -> i64 {
    let len = nums.len();
    let mut cnt = vec![0; len];
    let mut ans = 0;
    for l in 0..len {
        for j in 0..l {
            if nums[j] < nums[l] {
                ans += cnt[j];
            }
        }
        let mut z = 0;  // z是j之前小于nums[l]的个数
        for j in 0..l {
            if nums[j] > nums[l] {
                cnt[j] += z;
            }
            if nums[j] < nums[l] {
                z += 1;
            }
        }
    }
    ans
}

pub fn count_quadruplets3(nums: Vec<i32>) -> i64 {
    let len = nums.len();
    let mut result = 0;
    let mut cnt = vec![0; len];
    for l in 0..len {
        let mut z = 0;
        for j in 0..l {
            if nums[l] > nums[j] {
                result += cnt[j];
                z += 1;
            } else {
                cnt[j] += z;
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i64) {
        assert_eq!(func(vec![3, 9, 5, 4, 8, 2, 1, 10, 7, 6]), 7);
        assert_eq!(func(vec![1, 3, 2, 4, 5]), 2);
        assert_eq!(func(vec![1, 2, 3, 4]), 0);
    }
    test(count_quadruplets);
    test(count_quadruplets2);
    test(count_quadruplets3);
}
