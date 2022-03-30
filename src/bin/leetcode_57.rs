//! 插入区间

pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let idx = intervals.binary_search_by_key(&new_interval[0], |x| x[0]).unwrap_or_else(|x| x);
    intervals.insert(idx, new_interval);
    ans.push(intervals.remove(0));
    for i in intervals {
        let last = ans.last_mut().unwrap();
        if i[0] <= last[1] {
            if i[1] > last[1] {
                last[1] = i[1]
            }
        } else {
            ans.push(i);
        }
    }
    ans
}

fn main() {
    assert_eq!(insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]), vec![vec![1, 5], vec![6, 9]]);
    assert_eq!(insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![4, 8]), vec![vec![1, 2], vec![3, 10], vec![12, 16]]);
}
