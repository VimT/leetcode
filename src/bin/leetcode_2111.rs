//! 使数组 K 递增的最少操作次数

pub fn k_increasing(arr: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut result = 0;
    let len = arr.len();
    if k as usize == len { return 0; }
    for start in 0..k {
        let mut dp = vec![arr[start]];
        let mut idx = 0;
        let mut sublen = 1;
        for i in (start + k..len).step_by(k) {
            sublen += 1;
            let num = arr[i];
            if num >= dp[idx] {
                dp.push(num);
                idx += 1;
            } else {
                let k = dp.binary_search(&(num + 1)).unwrap_or_else(|x| x);
                dp[k] = num;
            }
        }
        result += sublen - dp.len();
    }
    result as i32
}

fn main() {
    assert_eq!(k_increasing(vec![2, 2, 2, 2, 2, 1, 1, 4, 4, 3, 3, 3, 3, 3], 1), 4);
    assert_eq!(k_increasing(vec![4, 1, 5, 2, 6, 2], 2), 0);
    assert_eq!(k_increasing(vec![5, 4, 3, 2, 1], 1), 4);
    assert_eq!(k_increasing(vec![4, 1, 5, 2, 6, 2], 3), 2);
}