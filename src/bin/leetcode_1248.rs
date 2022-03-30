//! 统计「优美子数组」

pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    let mut odd = vec![0; len + 2];
    let mut result = 0;
    let mut cnt = 0;
    for i in 0..len {
        if nums[i] & 1 == 1 {
            cnt += 1;
            odd[cnt] = i as i32;
        }
    }
    let k = k as usize;
    odd[0] = -1;
    odd[cnt + 1] = len as i32;
    if cnt < k {
        return 0;
    }
    for i in 1..=cnt + 1 - k {
        result += (odd[i] - odd[i - 1]) * (odd[i + k] - odd[i + k - 1]);
    }
    result
}

fn main() {
    assert_eq!(number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2), 16);
    assert_eq!(number_of_subarrays(vec![1, 1, 2, 1, 1], 3), 2);
    assert_eq!(number_of_subarrays(vec![2, 4, 6], 1), 0);
    assert_eq!(number_of_subarrays(vec![2, 4, 6], 3), 0);
}