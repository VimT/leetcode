//! 组合总和 III


pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    fn inner(k: usize, n: i32, current: &mut Vec<i32>, last: i32, ans: &mut Vec<Vec<i32>>) {
        if current.len() == k {
            if n == 0 {
                ans.push(current.clone());
            }
            return;
        }
        for i in last..=9 {
            if n >= i {
                current.push(i as i32);
                inner(k, n - i, current, i + 1, ans);
                current.pop();
            }
        }
    }
    let mut ans = vec![];
    inner(k as usize, n, &mut vec![], 1, &mut ans);
    ans
}

fn main() {
    assert_eq!(combination_sum3(3, 7), vec![vec![1, 2, 4]]);
    assert_eq!(combination_sum3(3, 9), vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]);
    assert_eq!(combination_sum3(4, 1).is_empty(), true);
}
