//! 使数组中的所有元素都等于零

pub fn check_array(nums: Vec<i32>, k: i32) -> bool {
    let len = nums.len();
    let mut diff = vec![0; len + 1];
    diff[0] = nums[0];
    for i in 1..len {
        diff[i] = nums[i] - nums[i - 1];
    }
    let mut cur = 0;
    let k = k as usize;
    for i in 0..len {
        cur += diff[i];
        if cur > 0 {
            if i + k > len { return false; }
            diff[i + k] += cur;
            cur = 0;
        } else if cur < 0 {
            return false;
        }
    }
    true
}

/// 双指针，空间复杂度 O(1)
pub fn check_array2(mut nums: Vec<i32>, k: i32) -> bool {
    let len = nums.len();
    let mut sum = 0;
    let k = k as usize;
    let mut left = 0;
    for right in 0..len {
        if right >= k {
            sum -= nums[left];
            left += 1;
        }
        nums[right] -= sum;
        if nums[right] < 0 || (right + k > len && nums[right] > 0) { return false; }
        if right + k <= len {
            sum += nums[right];
        }
    }
    true
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, k: i32) -> bool) {
        assert_eq!(func(vec![60, 72, 87, 89, 63, 52, 64, 62, 31, 37, 57, 83, 98, 94, 92, 77, 94, 91, 87, 100, 91, 91, 50, 26], 4), true);
        assert_eq!(func(vec![2, 2, 3, 1, 1, 0], 3), true);
        assert_eq!(func(vec![1, 3, 1, 1], 2), false);
    }
    test(check_array);
    test(check_array2);
}
