//! 长度最小的子数组

//! minimum-size-subarray-sum
pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len == 0 { return 0; }
    let mut sm = vec![0; len + 1];
    let mut ss = 0;
    for i in 0..len {
        ss += nums[i];
        sm[i + 1] = ss;
    }
    let mut ans = len + 1;
    for end in (1..=len).rev() {
        for start in 0..end {
            if sm[end] - sm[start] >= s {
                ans = ans.min(end - start)
            } else {
                break;
            }
        }
    }

    (if ans == len + 1 { 0 } else { ans }) as i32
}

pub fn min_sub_array_len_bisect(s: i32, nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len == 0 { return 0; }
    let mut sm = vec![0; len + 1];
    let mut ss = 0;
    for i in 0..len {
        ss += nums[i];
        sm[i + 1] = ss;
    }
    let mut ans = len + 1;
    for start in 0..=len {
        let target = s + sm[start];
        let end = sm.binary_search(&target).unwrap_or_else(|x| x);
        if end <= len {
            ans = ans.min(end - start);
        }
    }

    (if ans == len + 1 { 0 } else { ans }) as i32
}


pub fn min_sub_array_len_double_point(s: i32, nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut start = 0;
    let mut end = 0;
    let len = nums.len();
    if len == 0 { return 0; }
    let mut ans = len + 1;
    while end < len {
        sum += nums[end];
        while sum >= s {
            ans = ans.min(end - start + 1);
            sum -= nums[start];
            start += 1;
        }
        end += 1;
    }

    (if ans == len + 1 { 0 } else { ans }) as i32
}


fn main() {
    fn test(func: fn(target: i32, nums: Vec<i32>) -> i32) {
        assert_eq!(func(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(func(4, vec![1, 4, 4]), 1);
        assert_eq!(func(11, vec![1, 1, 1, 1, 1, 1, 1, 1]), 0);
    }
    test(min_sub_array_len);
    test(min_sub_array_len_bisect);
    test(min_sub_array_len_double_point);
}
