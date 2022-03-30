//! 第k个排列

pub fn get_permutation(n: i32, mut k: i32) -> String {
    let mut sum = (1..n).fold(1, |x, y| x * y);
    let mut ans = vec![];
    let mut nums: Vec<i32> = (1..=n).collect();
    for i in (0..n).rev() {
        let idx = (k - 1) / sum;
        ans.push(nums.remove(idx as usize));
        k -= idx * sum;
        if i > 0 { sum /= i; }
    }
    ans.into_iter().map(|x| x.to_string()).collect()
}

fn main() {
    assert_eq!(get_permutation(4, 9), "2314".to_string());
    assert_eq!(get_permutation(3, 3), "213".to_string());
    assert_eq!(get_permutation(1, 1), "1".to_string());
    assert_eq!(get_permutation(2, 2), "21".to_string());
    assert_eq!(get_permutation(3, 6), "321".to_string());
    assert_eq!(get_permutation(9, 1), "123456789".to_string());
}

