//! 展开二维向量

struct Vector2D {
    vec: Vec<Vec<i32>>,
    i: usize,
    j: usize,
}


impl Vector2D {
    fn new(vec: Vec<Vec<i32>>) -> Self {
        let mut i = 0;
        while i < vec.len() && vec[i].is_empty() {
            i += 1;
        }
        Self { vec, i, j: 0 }
    }


    fn next(&mut self) -> i32 {
        let result = self.vec[self.i][self.j];
        self.j += 1;
        while self.i < self.vec.len() && self.j == self.vec[self.i].len() {
            self.i += 1;
            self.j = 0;
        }
        return result;
    }

    fn has_next(&self) -> bool {
        self.i < self.vec.len() && self.j < self.vec[self.i].len()
    }
}


fn main() {
    let mut iter = Vector2D::new(vec![vec![1, 2], vec![3], vec![4]]);
    assert_eq!(iter.next(), 1);
    assert_eq!(iter.next(), 2);
    assert_eq!(iter.next(), 3);
    assert_eq!(iter.has_next(), true);
    assert_eq!(iter.has_next(), true);
    assert_eq!(iter.next(), 4);
    assert_eq!(iter.has_next(), false);
}
