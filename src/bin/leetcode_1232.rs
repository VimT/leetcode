//! 缀点成线

pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
    for i in 2..coordinates.len() {
        if (coordinates[1][1] - coordinates[0][1]) * (coordinates[i][0] - coordinates[0][0]) != (coordinates[i][1] - coordinates[0][1]) * (coordinates[1][0] - coordinates[0][0]) {
            return false;
        }
    }
    true
}

fn main() {
    fn test(func: fn(coordinates: Vec<Vec<i32>>) -> bool) {
        assert_eq!(func(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7]]), true);
        assert_eq!(func(vec![vec![1, 1], vec![2, 2], vec![3, 4], vec![4, 5], vec![5, 6], vec![7, 7]]), false);
    }
    test(check_straight_line);
}
