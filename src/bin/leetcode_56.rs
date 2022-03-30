//! 合并区间

pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
    let mut ans = vec![];
    for i in intervals {
        if ans.len() == 0 {
            ans.push(i);
        } else {
            let last = ans.last_mut().unwrap();
            if i[0] <= last[1] {
                if i[1] > last[1] {
                    last[1] = i[1]
                }
            } else {
                ans.push(i);
            }
        }
    }
    ans
}

fn main() {
    assert_eq!(merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]), vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
    assert_eq!(merge(vec![vec![1, 4], vec![4, 5]]), vec![vec![1, 5]]);
}
