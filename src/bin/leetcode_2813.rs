//! 子序列最大优雅度

/// 反悔贪心
/// 先选最大的k个 profit，然后考虑贪心的增加 distinct_categories
pub fn find_maximum_elegance(mut items: Vec<Vec<i32>>, k: i32) -> i64 {
    let len = items.len();
    items.sort_unstable_by_key(|x| -x[0]);
    let mut used_category = vec![false; len];
    let k = k as usize;
    let mut distinct_categories = 0;
    let mut psum = 0;
    let mut nums = vec![];
    for item in &items[..k] {
        let (p, c) = (item[0] as i64, item[1] as usize - 1);
        if used_category[c] { nums.push(p); } else {
            used_category[c] = true;
            distinct_categories += 1;
        }
        psum += p;
    }

    let mut result = psum + distinct_categories * distinct_categories;
    for item in &items[k..] {
        let (p, c) = (item[0] as i64, item[1] as usize - 1);
        if !used_category[c] {
            if nums.is_empty() { break; }
            psum += p - nums.pop().unwrap();
            used_category[c] = true;
            distinct_categories += 1;
            result = result.max(psum + distinct_categories * distinct_categories);
        }
    }

    result
}

fn main() {
    fn test(func: fn(items: Vec<Vec<i32>>, k: i32) -> i64) {
        assert_eq!(func(vec![vec![3, 2], vec![5, 1], vec![10, 1]], 2), 17);
        assert_eq!(func(vec![vec![3, 1], vec![3, 1], vec![2, 2], vec![5, 3]], 3), 19);
        assert_eq!(func(vec![vec![1, 1], vec![2, 1], vec![3, 1]], 3), 7);
        assert_eq!(func(vec![vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![10, 1], vec![3, 2], vec![3, 3], vec![3, 4], vec![3, 5], vec![3, 6], vec![3, 7], vec![3, 8], vec![3, 9], vec![3, 10], vec![3, 11]], 10), 137);
    }
    test(find_maximum_elegance);
}
