//! 强整数

use std::collections::HashSet;

pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
    let mut set = HashSet::new();
    let conv = |num: i32| if num == 1 { 0 } else { (bound as f64).log(num as f64) as i32 };
    for i in 0..=conv(x) {
        let xx = x.pow(i as u32);
        for j in 0..=conv(y) {
            let yy = y.pow(j as u32);
            if xx + yy > bound { break; }
            set.insert(xx + yy);
        }
    }
    set.into_iter().collect()
}

fn main() {
    fn order(mut x: Vec<i32>) -> Vec<i32> {
        x.sort_unstable();
        x
    }
    assert_eq!(order(powerful_integers(81, 21, 900000)), order(vec![540702, 531442, 2, 531462, 6582, 102, 194562, 522, 201042, 194482, 9262, 6562, 82, 15822, 22, 725922, 442, 7002, 9342, 531882]));
    assert_eq!(order(powerful_integers(2, 3, 10)), vec![2, 3, 4, 5, 7, 9, 10]);
    assert_eq!(order(powerful_integers(3, 5, 15)), vec![2, 4, 6, 8, 10, 14]);
}
