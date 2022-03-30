//! 组合

pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    fn inner(start: i32, end: i32, k: i32) -> Vec<Vec<i32>> {
        if k == 1 {
            return (start..=end).map(|x| vec![x]).collect();
        }
        let mut ans = vec![];
        for i in start..=end - k + 1 {
            for mut t in inner(i + 1, end, k - 1) {
                t.insert(0, i);
                ans.push(t);
            }
        }
        ans
    }

    inner(1, n, k)
}

fn main() {
    assert_eq!(combine(4, 2), [[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]);
    assert_eq!(combine(5, 5), [[1, 2, 3, 4, 5]]);
    assert_eq!(combine(5, 1), [[1], [2], [3], [4], [5]]);
    assert_eq!(combine(5, 3), [[1, 2, 3], [1, 2, 4], [1, 2, 5], [1, 3, 4], [1, 3, 5], [1, 4, 5], [2, 3, 4], [2, 3, 5], [2, 4, 5], [3, 4, 5]]);
}