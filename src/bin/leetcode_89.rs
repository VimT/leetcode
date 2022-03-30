//! 格雷编码

pub fn gray_code(n: i32) -> Vec<i32> {
    let mut ans = vec![0];
    let mut head = 1;
    for i in 0..n {
        for j in (0..ans.len()).rev() {
            ans.push(head + ans[j]);
        }
        head <<= 1;
    }
    ans
}

fn main() {
    assert_eq!(gray_code(2), vec![0, 1, 3, 2]);
    assert_eq!(gray_code(1), vec![0, 1]);
}
