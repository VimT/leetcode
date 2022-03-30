//! 射箭比赛中的最大得分

pub fn maximum_bob_points(num_arrows: i32, alice_arrows: Vec<i32>) -> Vec<i32> {
    fn dfs(a: &Vec<i32>, left: i32, b: &mut Vec<i32>, max: &mut i32, result: &mut Vec<i32>) {
        if b.len() == 12 {
            let mut score = 0;
            for i in 0..12 {
                if a[i] < b[i] { score += i as i32; }
            }
            if score > *max {
                *max = score;
                *result = b.clone();
                result[0] += left;
            }
            return;
        }
        let idx = b.len();
        if left >= a[idx] + 1 {
            b.push(a[idx] + 1);
            dfs(a, left - a[idx] - 1, b, max, result);
            b.pop();
        }
        b.push(0);
        dfs(a, left, b, max, result);
        b.pop();
    }
    let mut result = Vec::with_capacity(12);
    dfs(&alice_arrows, num_arrows, &mut vec![], &mut 0, &mut result);
    result
}

fn main() {
    assert_eq!(maximum_bob_points(89, vec![3, 2, 28, 1, 7, 1, 16, 7, 3, 13, 3, 5]), vec![21, 3, 0, 2, 8, 2, 17, 8, 4, 14, 4, 6]);
    assert_eq!(maximum_bob_points(9, vec![1, 1, 0, 1, 0, 0, 2, 1, 0, 1, 2, 0]), vec![0, 0, 0, 0, 1, 1, 0, 0, 1, 2, 3, 1]);
    assert_eq!(maximum_bob_points(3, vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2]), vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0]);
}
