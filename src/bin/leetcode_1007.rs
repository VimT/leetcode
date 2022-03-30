//! 行相等的最少多米诺旋转

pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
    let mut result = i32::MAX;
    let len = tops.len();
    'out: for i in 1..=6 {
        let mut cnt1 = 0;
        let mut cnt2 = 0;
        for j in 0..len {
            match (tops[j] == i, bottoms[j] == i) {
                (true, false) => { cnt1 += 1; }
                (false, true) => { cnt2 += 1; }
                (false, false) => { continue 'out; }
                _ => (),
            }
        }
        result = result.min(cnt1.min(cnt2));
    }
    if result == i32::MAX { -1 } else { result }
}

fn main() {
    assert_eq!(min_domino_rotations(vec![2, 1, 2, 4, 2, 2], vec![5, 2, 6, 2, 3, 2]), 2);
    assert_eq!(min_domino_rotations(vec![3, 5, 1, 2, 3], vec![3, 6, 3, 3, 4]), -1);
}
