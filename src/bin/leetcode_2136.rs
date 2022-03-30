//! 全部开花的最早一天

pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
    let mut g: Vec<(i32, i32)> = grow_time.into_iter().zip(plant_time).collect();
    g.sort_unstable();
    let mut plat_day = 0;
    let mut max_open_day = 0;
    for (grow, plat) in g.into_iter().rev() {
        plat_day += plat;
        max_open_day = max_open_day.max(plat_day + grow);
    }
    max_open_day
}

fn main() {
    assert_eq!(earliest_full_bloom(vec![1, 4, 3], vec![2, 3, 1]), 9);
    assert_eq!(earliest_full_bloom(vec![1, 2, 3, 2], vec![2, 1, 2, 1]), 9);
    assert_eq!(earliest_full_bloom(vec![1], vec![1]), 2);
}
