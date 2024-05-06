//! 找出唯一性数组的中位数

/// 二分答案，distinct值 <= upper 的子数组有多少个？
pub fn median_of_uniqueness_array(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let k = ((len * (len + 1)) / 2 + 1) / 2;
    let mut left = 1;
    let mut right = len;
    let mut cnt = vec![0; nums.iter().max().copied().unwrap() as usize + 1];
    while left < right {
        let mid = (left + right) / 2;
        cnt.fill(0);
        let mut distinct = 0;
        let mut l = 0;
        let mut sum = 0;
        for (r, &num) in nums.iter().enumerate() {
            cnt[num as usize] += 1;
            if cnt[num as usize] == 1 {
                distinct += 1;
            }
            while distinct > mid {
                let num = nums[l] as usize;
                cnt[num] -= 1;
                if cnt[num] == 0 {
                    distinct -= 1;
                }
                l += 1;
            }
            sum += r + 1 - l;
            if sum >= k {
                break;
            }
        }
        if sum >= k {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left as i32
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![1, 2, 3]), 1);
        assert_eq!(func(vec![3, 4, 3, 4, 5]), 2);
        assert_eq!(func(vec![4, 3, 5, 4]), 2);
    }
    test(median_of_uniqueness_array);
}
