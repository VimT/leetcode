//! 查找大小为 M 的最新分组

use std::collections::BTreeSet;

/// 倒序插0。也可以用并查集维护连通分量做
pub fn find_latest_step(arr: Vec<i32>, m: i32) -> i32 {
    if m == arr.len() as i32 { return m; }
    let mut set = BTreeSet::new();
    set.insert(0);
    set.insert(arr.len() as i32 + 1);
    for (i, num) in arr.into_iter().enumerate().rev() {
        let left = *set.range(..num).last().unwrap();
        let right = *set.range(num..).next().unwrap();
        if num - left - 1 == m || right - num - 1 == m {
            return i as i32;
        }
        set.insert(num);
    }
    -1
}

/// 每操作一次，新增的 1 可能会有如下三种情况：
///   左右都是 0。此时该位置作为 新增段独立存在。
///   仅有左边或者右边。此时该位置 会将某个旧段的长度加 1。
///   左右都是 1。此时 该位置会将两个旧段合并成一个新段。
/// 维护长度为m的1段数量
pub fn find_latest_step2(arr: Vec<i32>, m: i32) -> i32 {
    let len = arr.len();
    let mut link = vec![0; len + 2]; // link[i] 表示一段1的另一个端点的位置。
    let mut cnt = 0;
    let mut result = -1;
    let m = m as usize;
    for (i, cur) in arr.into_iter().map(|x| x as usize).enumerate() {
        let left = if link[cur - 1] != 0 { link[cur - 1] } else { cur };
        let right = if link[cur + 1] != 0 { link[cur + 1] } else { cur };
        if cur - left == m { cnt -= 1; }
        if right - cur == m { cnt -= 1; }
        if right - left + 1 == m { cnt += 1; }
        if cnt > 0 { result = i as i32 + 1; }
        link[left] = right;
        link[right] = left;
    }
    result
}

fn main() {
    fn test(func: fn(arr: Vec<i32>, m: i32) -> i32) {
        assert_eq!(func(vec![3, 5, 1, 2, 4], 1), 4);
        assert_eq!(func(vec![3, 1, 5, 4, 2], 2), -1);
        assert_eq!(func(vec![1], 1), 1);
        assert_eq!(func(vec![2, 1], 2), 2);
    }
    test(find_latest_step);
    test(find_latest_step2);
}
