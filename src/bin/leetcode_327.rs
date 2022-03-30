//! 区间和的个数


pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
    let len = nums.len();
    let lower = lower as i64;
    let upper = upper as i64;
    let mut ans = 0;
    for i in 0..len {
        let mut sum: i64 = 0;
        for j in i..len {
            sum += nums[j] as i64;
            if sum >= lower && sum <= upper {
                ans += 1;
            }
        }
    }
    ans
}

// error
pub fn count_range_sum_merge_sort(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
    fn merge_sort(sum: &mut Vec<i64>, lower: i64, upper: i64, left: usize, right: usize) -> i32 {
        if left == right { return 0; }
        let mid = (left + right) / 2;
        let n1 = merge_sort(sum, lower, upper, left, mid);
        let n2 = merge_sort(sum, lower, upper, mid + 1, right);
        let mut ret = n1 + n2;

        let mut i = left;
        let mut l = mid + 1;
        let mut r = mid + 1;
        while i <= mid {
            while l <= right && sum[l] - sum[i] < lower { l += 1; }
            while r <= right && sum[r] - sum[i] <= upper { r += 1; }
            ret += (r - l) as i32;
            i += 1;
        }

        let mut sorted = vec![0; right - left + 1];
        let mut p1 = left;
        let mut p2 = mid + 1;
        let mut p = 0;
        while p1 <= mid || p2 <= right {
            if p1 > mid {
                sorted[p] = sum[p2];
                p += 1;
                p2 += 1;
            } else if p2 > right {
                sorted[p] = sum[p1];
                p += 1;
                p1 += 1;
            } else {
                if sum[p1] < sum[p2] {
                    sorted[p] = sum[p1];
                    p += 1;
                    p1 += 1;
                } else {
                    sorted[p] = sum[p2];
                    p += 1;
                    p2 += 1;
                }
            }
        }
        for i in 0..sorted.len() {
            sum[left + i] = sorted[i];
        }
        ret
    }
    let mut s: i64 = 0;
    let len = nums.len();
    let mut sum = vec![0; len];
    let mut ans = 0;
    for i in 0..len {
        s += nums[i] as i64;
        sum[i] = s;
        if nums[i] >= lower && nums[i] <= upper {
            ans += 1;
        }
    }
    ans + merge_sort(&mut sum, lower as i64, upper as i64, 0, len - 1)
}

pub fn count_range_sum_best(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
    if lower > upper { return 0; }
    let (mut res, mut rec, mut a) = (0, 0, vec![]);
    let (mut lower, mut upper) = (lower as i64, upper as i64);
    for num in nums.iter() {
        a.insert(a.binary_search(&rec).unwrap_or_else(|x| x), rec);
        rec -= *num;
        lower -= *num as i64;
        upper -= *num as i64;

        // if there are multiple matches, then the first of the matches will be returned
        let left = a.binary_search_by(|x| (*x as i64).cmp(&lower)).unwrap_or_else(|x| x);
        let right = a.binary_search_by(|x| (*x as i64).cmp(&(upper + 1))).unwrap_or_else(|x| x);
        res += right as i32 - left as i32; // right - left >= 0
    }
    res
}

fn main() {
    fn test(func: fn(nums: Vec<i32>, lower: i32, upper: i32) -> i32) {
        assert_eq!(func(vec![-2, 5, -1], -2, 2), 3);
        assert_eq!(func(vec![0], 0, 0), 1);
    }
    test(count_range_sum);
    test(count_range_sum_best);
    test(count_range_sum_merge_sort);
}
