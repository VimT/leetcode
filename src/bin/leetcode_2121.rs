//! 相同元素的间隔之和

pub fn get_distances(arr: Vec<i32>) -> Vec<i64> {
    let mut arr: Vec<(i32, usize)> = arr.into_iter().enumerate().map(|x| (x.1, x.0)).collect();
    let len = arr.len();
    let mut result = vec![0; len];
    arr.sort_unstable();
    let mut i = 0;
    while i < len {
        let mut right = i + 1;
        let num = arr[i].0;
        while right < len && arr[right].0 == num { right += 1; }
        let mut presum = vec![0; right - i];
        for j in i..right {
            presum[j - i] = if j > i { presum[j - i - 1] } else { 0 } + arr[j].1;
        }
        for j in i..right {
            result[arr[j].1] = (presum[right - i - 1] - presum[j - i] - (right - j - 1) * arr[j].1 + arr[j].1 * (j + 1 - i) - presum[j - i]) as i64;
        }
        i = right;
    }
    result
}

fn main() {
    assert_eq!(get_distances(vec![2, 1, 3, 1, 2, 3, 3]), vec![4, 2, 7, 2, 4, 4, 5]);
    assert_eq!(get_distances(vec![10, 5, 10, 10]), vec![5, 0, 3, 4]);
}
