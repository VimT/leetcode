//! 摧毁小行星

pub fn asteroids_destroyed(mass: i32, mut asteroids: Vec<i32>) -> bool {
    asteroids.sort_unstable();
    let mut ma = mass as i64;
    for num in asteroids {
        if ma < num as i64 { return false; } else { ma += num as i64; }
    }
    true
}

fn main() {
    assert_eq!(asteroids_destroyed(10, vec![3, 9, 19, 5, 21]), true);
    assert_eq!(asteroids_destroyed(5, vec![4, 9, 23, 4]), false);
}
