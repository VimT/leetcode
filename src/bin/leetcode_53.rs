//! 最大子数组和

/// dp[i] = max(dp[i-1] + nums[i], nums[i])
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut pre = 0;
    let mut ans = nums[0];
    for i in nums {
        pre = (pre + i).max(i);
        ans = ans.max(pre);
    }
    ans
}

/// 类似线段树push_up
pub fn max_sub_array_push_up(nums: Vec<i32>) -> i32 {
    struct Status {
        // [l, r] 以l为左端点的最大子段和
        l_sum: i32,
        // [l, r] 以r为右端点的最大子段和
        r_sum: i32,
        // [l, r] 内最大子段和
        m_sum: i32,
        // [l, r] 的区间和
        i_sum: i32,
    }
    fn push_up(l: Status, r: Status) -> Status {
        let i_sum = l.i_sum + r.i_sum;
        let l_sum = l.l_sum.max(l.i_sum + r.l_sum);
        let r_sum = r.r_sum.max(r.i_sum + l.r_sum);
        let m_sum = l.m_sum.max(r.m_sum).max(l.r_sum + r.l_sum);
        return Status { l_sum, r_sum, m_sum, i_sum };
    }

    fn get(nums: &Vec<i32>, l: usize, r: usize) -> Status {
        if l == r { return Status { l_sum: nums[l], r_sum: nums[l], m_sum: nums[l], i_sum: nums[l] }; }
        let m = (l + r) >> 1;
        let l_sub = get(nums, l, m);
        let r_sub = get(nums, m + 1, r);
        push_up(l_sub, r_sub)
    }

    let len = nums.len();
    return get(&nums, 0, len - 1).m_sum;
}


fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(func(vec![1]), 1);
        assert_eq!(func(vec![5, 4, -1, 7, 8]), 23);
    }
    test(max_sub_array);
    test(max_sub_array_push_up);
}
