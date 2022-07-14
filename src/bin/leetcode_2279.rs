//! 装满石头的背包的最大数量

pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, mut additional_rocks: i32) -> i32 {
    let mut left: Vec<i32> = capacity.into_iter().zip(rocks.into_iter()).map(|(a, b)| a - b).collect();
    left.sort_unstable();
    let mut cnt = 0;
    for space in left {
        if space > additional_rocks {
            break;
        }
        additional_rocks -= space;
        cnt += 1;
    }
    cnt
}

fn main() {
    fn test(func: fn(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32) {
        assert_eq!(func(vec![2, 3, 4, 5], vec![1, 2, 4, 4], 2), 3);
        assert_eq!(func(vec![10, 2, 2], vec![2, 2, 0], 100), 3);
    }
    test(maximum_bags);
}
