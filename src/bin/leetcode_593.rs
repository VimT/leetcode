//! 有效的正方形

struct Vector {
    x: i32,
    y: i32,
    length: f64,
}

impl Vector {
    fn new(p1: &Vec<i32>, p2: &Vec<i32>) -> Self {
        let x = p1[0] - p2[0];
        let y = p1[1] - p2[1];
        Vector { x, y, length: ((x * x) as f64 + (y * y) as f64).sqrt() }
    }
}

impl Vector {
    fn is_vertical(&self, rhs: &Vector) -> bool {
        self.x * rhs.x + self.y * rhs.y == 0 && self.length == rhs.length
    }
}

pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
    let lines = [Vector::new(&p1, &p2), Vector::new(&p1, &p3), Vector::new(&p1, &p4), Vector::new(&p2, &p3), Vector::new(&p2, &p4), Vector::new(&p3, &p4)];
    let mut cnt = 0;
    for i in 0..6 {
        if lines[i].x == 0 && lines[i].y == 0 { return false; }
        let mut vertical = 0;
        for j in 0..6 {
            if i == j { continue; }
            if lines[i].is_vertical(&lines[j]) {
                vertical += 1;
            }
        }
        if vertical == 2 { cnt += 1; }
    }
    cnt == 4
}

fn main() {
    assert_eq!(valid_square(vec![0, 0], vec![1, 1], vec![1, 1], vec![1, 0]), false);
    assert_eq!(valid_square(vec![0, 0], vec![0, 4], vec![5, 4], vec![0, 5]), false);
    assert_eq!(valid_square(vec![1, 0], vec![-1, 0], vec![0, 1], vec![0, -1]), true);
    assert_eq!(valid_square(vec![0, 0], vec![1, 1], vec![1, 0], vec![0, 1]), true);
    assert_eq!(valid_square(vec![0, 0], vec![1, 1], vec![1, 0], vec![0, 12]), false);
}
