//! 子矩形查询

struct SubrectangleQueries {
    rectangle: Vec<Vec<i32>>,
    history_update: Vec<(i32, i32, i32, i32, i32)>,
}


impl SubrectangleQueries {
    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        Self { rectangle, history_update: vec![] }
    }

    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        self.history_update.push((row1, col1, row2, col2, new_value));
    }

    fn get_value(&self, row: i32, col: i32) -> i32 {
        for &(r1, c1, r2, c2, val) in self.history_update.iter().rev() {
            if row >= r1 && row <= r2 && col >= c1 && col <= c2 {
                return val;
            }
        }
        return self.rectangle[row as usize][col as usize];
    }
}

fn main() {
    let mut sq = SubrectangleQueries::new(vec![vec![1, 2, 1], vec![4, 3, 4], vec![3, 2, 1], vec![1, 1, 1]]);
    assert_eq!(sq.get_value(0, 2), 1);
    sq.update_subrectangle(0, 0, 3, 2, 5);
    assert_eq!(sq.get_value(0, 2), 5); // 返回 5
    assert_eq!(sq.get_value(3, 1), 5); // 返回 5
    sq.update_subrectangle(3, 0, 3, 2, 10);
    // 此次更新后矩形变为：
    // 5   5   5
    // 5   5   5
    // 5   5   5
    // 10  10  10
    assert_eq!(sq.get_value(3, 1), 10); // 返回 10
    assert_eq!(sq.get_value(0, 2), 5); // 返回 5
}
