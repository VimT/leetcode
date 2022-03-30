//! 找出第 k 小的距离对

pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    let len = nums.len();
    let mut left = 0;
    let mut right = nums[len - 1] - nums[0];
    while left < right {
        let mid = (left + right) / 2;
        let mut cnt = 0;
        let mut j = 1;
        for i in 0..len - 1 {
            while j < len && nums[j] - nums[i] <= mid { j += 1; }
            cnt += j - i - 1;
        }
        if cnt as i32 >= k {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn main() {
    assert_eq!(smallest_distance_pair(vec![9, 10, 7, 10, 6, 1, 5, 4, 9, 8], 18), 2);
    assert_eq!(smallest_distance_pair(vec![1, 6, 1], 3), 5);
    assert_eq!(smallest_distance_pair(vec![1, 3, 1], 1), 0);
    assert_eq!(smallest_distance_pair(vec![1, 1, 1], 2), 0);
}
