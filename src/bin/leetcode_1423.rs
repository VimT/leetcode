//! 可获得的最大点数

pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut ps = vec![0; k + 1];
    let mut ss = vec![0; k + 1];
    let len = card_points.len();
    let mut cur = 0;
    for i in 1..=k {
        cur += card_points[i - 1] as i64;
        ps[i] = cur;
    }
    cur = 0;
    for i in 1..=k {
        cur += card_points[len - i] as i64;
        ss[i] = cur;
    }
    let mut ans = 0;
    for i in 0..=k {
        ans = ans.max(ps[i] + ss[k - i]);
    }
    ans as i32
}


fn main() {
    assert_eq!(max_score(vec![1, 2, 3, 4, 5, 6, 1], 3), 12);
    assert_eq!(max_score(vec![2, 2, 2], 2), 4);
    assert_eq!(max_score(vec![9, 7, 7, 9, 7, 7, 9], 7), 55);
}
