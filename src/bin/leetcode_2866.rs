//! 美丽塔 II

/// 单调栈，同时维护前缀/后缀和
pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
    let len = max_heights.len();
    let mut result = 0;
    let mut s = vec![];
    let mut right = vec![0; len];
    let mut cnt = vec![1; len]; // cnt[i] 表示 单调增栈中 max_heights[i] 被pop的时候，cur_sum 要减去差值多少次
    let mut cur_sum = 0;
    for i in (0..len).rev() {
        while !s.is_empty() && max_heights[*s.last().unwrap()] >= max_heights[i] {
            let idx = s.pop().unwrap();
            cur_sum -= cnt[idx] * (max_heights[idx] - max_heights[i]) as i64;
            cnt[i] += cnt[idx];
        }
        s.push(i);
        cur_sum += max_heights[i] as i64;
        right[i] = cur_sum;
    }
    cur_sum = 0;
    cnt.fill(1);
    s.clear();
    for i in 0..len {
        while !s.is_empty() && max_heights[*s.last().unwrap()] >= max_heights[i] {
            let idx = s.pop().unwrap();
            cur_sum -= cnt[idx] * (max_heights[idx] - max_heights[i]) as i64;
            cnt[i] += cnt[idx];
        }
        s.push(i);
        result = result.max(cur_sum + right[i]);
        cur_sum += max_heights[i] as i64;
    }
    result
}

/// 灵茶，不用维护cnt次数的版本
pub fn maximum_sum_of_heights2(max_heights: Vec<i32>) -> i64 {
    let len = max_heights.len();
    let mut st = Vec::with_capacity(len);
    let mut s = 0;
    let mut suf = vec![0; len + 1];
    for i in (0..len).rev() {
        while !st.is_empty() && max_heights[*st.last().unwrap() as usize] >= max_heights[i] {
            let idx = st.pop().unwrap();
            s -= max_heights[idx as usize] as i64 * (st.last().copied().unwrap_or(len as i64) - idx);
        }
        s += max_heights[i] as i64 * (st.last().copied().unwrap_or(len as i64) - i as i64);
        st.push(i as i64);
        suf[i] = s;
    }
    let mut result = s;
    s = 0;
    st.clear();
    for i in 0..len {
        while !st.is_empty() && max_heights[*st.last().unwrap() as usize] >= max_heights[i] {
            let idx = st.pop().unwrap();
            s -= max_heights[idx as usize] as i64 * (idx - st.last().copied().unwrap_or(-1));
        }
        s += max_heights[i] as i64 * (i as i64 - st.last().copied().unwrap_or(-1));
        result = result.max(s + suf[i + 1]);
        st.push(i as i64);
    }
    result
}


fn main() {
    fn test(func: fn(max_heights: Vec<i32>) -> i64) {
        assert_eq!(func(vec![5, 3, 4, 1, 1]), 13);
        assert_eq!(func(vec![6, 5, 3, 9, 2, 7]), 22);
        assert_eq!(func(vec![3, 2, 5, 5, 2, 3]), 18);
    }
    test(maximum_sum_of_heights);
    test(maximum_sum_of_heights2);
}
