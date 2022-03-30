//! 设置交集大小至少为2

///贪心，按s升序，e降序排序。从后往前取，每次都取前n个
pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_unstable_by_key(|x| (x[0], -x[1]));
    let len = intervals.len();
    let mut todo = vec![2; len];
    let mut result = 0;
    while !intervals.is_empty() {
        let interval = intervals.pop().unwrap();
        let t = todo.pop().unwrap();
        let s = interval[0];
        for x in s..s + t {
            for (i, int) in intervals.iter().enumerate() {
                if todo[i] > 0 && x <= int[1] {
                    todo[i] -= 1;
                }
            }
            result += 1;
        }
    }
    result
}

fn main() {
    assert_eq!(intersection_size_two(vec![vec![1, 3], vec![1, 4], vec![2, 5], vec![3, 5]]), 3);
    assert_eq!(intersection_size_two(vec![vec![1, 2], vec![2, 3], vec![2, 4], vec![4, 5]]), 5);
}
