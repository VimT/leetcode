//! 两个非重叠子数组的最大和

pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
    let len = nums.len();
    let first_len = first_len as usize;
    let second_len = second_len as usize;
    let mut f1 = vec![0; len];
    let mut l1 = vec![0; len];
    let mut f2 = vec![0; len];
    let mut l2 = vec![0; len];
    let mut cur: i32 = nums[..first_len].iter().sum();
    l1[first_len - 1] = cur;
    for i in first_len..len {
        cur -= nums[i - first_len];
        cur += nums[i];
        l1[i] = l1[i - 1].max(cur);
    }
    cur = nums[..second_len].iter().sum();
    l2[second_len - 1] = cur;
    for i in second_len..len {
        cur -= nums[i - second_len];
        cur += nums[i];
        l2[i] = l2[i - 1].max(cur);
    }
    cur = nums[len - first_len..].iter().sum();
    f1[len - first_len] = cur;
    for i in (0..len - first_len).rev() {
        cur -= nums[i + first_len];
        cur += nums[i];
        f1[i] = f1[i + 1].max(cur);
    }
    cur = nums[len - second_len..].iter().sum();
    f2[len - second_len] = cur;
    for i in (0..len - second_len).rev() {
        cur -= nums[i + second_len];
        cur += nums[i];
        f2[i] = f2[i + 1].max(cur);
    }
    let mut result = 0;
    for i in 1..len {
        result = result.max(f1[i] + l2[i - 1]).max(f2[i] + l1[i - 1]);
    }
    result
}

fn main() {
    assert_eq!(max_sum_two_no_overlap(vec![0, 6, 5, 2, 2, 5, 1, 9, 4], 1, 2), 20);
    assert_eq!(max_sum_two_no_overlap(vec![3, 8, 1, 3, 2, 1, 8, 9, 0], 3, 2), 29);
    assert_eq!(max_sum_two_no_overlap(vec![2, 1, 5, 6, 0, 9, 5, 0, 3, 8], 4, 3), 31);
}
