//! 标记所有元素后数组的分数


pub fn find_score(nums: Vec<i32>) -> i64 {
    let mut nums: Vec<(i32, usize)> = nums.into_iter().zip(0..).collect();
    nums.sort_unstable();
    let len = nums.len();
    let mut seen = vec![false; len];
    let mut result = 0;
    for (num, pos) in nums {
        if !seen[pos] {
            result += num as i64;
            seen[pos] = true;
            if pos > 0 { seen[pos - 1] = true; }
            if pos + 1 < len { seen[pos + 1] = true; }
        }
    }
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i64) {
        assert_eq!(func(vec![1, 1, 1]), 2);
        assert_eq!(func(vec![730, 1721, 1993, 1532, 962, 519, 1365, 1307, 1992, 1623, 1579, 1735, 1015, 1579, 1003, 1277, 1255, 1254, 810, 1767, 206, 1837, 920, 1203, 1876, 521, 625, 483, 572, 922, 1936, 1014, 1835, 694, 19, 209, 1438, 127, 1716, 1272, 444, 1739, 148, 1580, 802, 1093, 1543, 452, 257, 103, 877, 749, 1418, 348, 1555, 1552, 818, 555, 1628, 547, 553, 1742, 1062, 1199, 1987, 818, 491, 1211]), 23202);
        assert_eq!(func(vec![2, 1, 3, 4, 5, 2]), 7);
        assert_eq!(func(vec![2, 3, 5, 1, 3, 2]), 5);
    }
    test(find_score);
}
