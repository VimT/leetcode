//! 平方数之和

pub fn judge_square_sum(c: i32) -> bool {
    let end = (c as f64 / 2.0).sqrt() as i32 + 1;
    for a in 0..=end {
        let c_a = c - a * a;
        let b = ((c_a) as f64).sqrt() as i32;
        if b * b == c_a {
            return true;
        }
    }
    false
}

fn main() {
    assert_eq!(judge_square_sum(16), true);
    assert_eq!(judge_square_sum(5), true);
    assert_eq!(judge_square_sum(3), false);
}
