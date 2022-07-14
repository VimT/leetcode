//! 设计 Excel 求和公式

use leetcode::svec;

#[derive(Copy, Clone)]
enum SumType {
    Single(usize, usize),
    Multi(usize, usize, usize, usize),
}

impl SumType {
    fn single_str(s: &str) -> (usize, usize) {
        (s[1..].parse::<usize>().unwrap() - 1, (s.as_bytes()[0] - b'A') as usize)
    }
    fn from_str(s: &str) -> Self {
        if s.contains(':') {
            let (a, b) = s.split_once(':').unwrap();
            let (i1, j1) = Self::single_str(a);
            let (i2, j2) = Self::single_str(b);
            SumType::Multi(i1, j1, i2, j2)
        } else {
            let (i, j) = Self::single_str(s);
            SumType::Single(i, j)
        }
    }
}

#[derive(Clone)]
enum Grid {
    Num(i32),
    Sum(Vec<SumType>),
}

struct Excel {
    mat: Vec<Vec<Grid>>,
}


impl Excel {
    fn new(height: i32, width: char) -> Self {
        let m = height as usize;
        let n = (width as u8 - b'A' + 1) as usize;
        Self { mat: vec![vec![Grid::Num(0); n]; m] }
    }


    fn set(&mut self, row: i32, column: char, val: i32) {
        let i = row as usize - 1;
        let j = (column as u8 - b'A') as usize;
        self.mat[i][j] = Grid::Num(val);
    }

    fn cache_get(&self, i: usize, j: usize, cache: &mut Vec<Vec<Option<i32>>>) -> i32 {
        if let Some(v) = cache[i][j] {
            return v;
        }
        let result = match &self.mat[i][j] {
            Grid::Num(v) => {
                *v
            }
            Grid::Sum(items) => {
                let mut sum = 0;
                for &item in items {
                    match item {
                        SumType::Single(i, j) => {
                            sum += self.cache_get(i, j, cache);
                        }
                        SumType::Multi(i1, j1, i2, j2) => {
                            for i in i1..=i2 {
                                for j in j1..=j2 {
                                    sum += self.cache_get(i, j, cache);
                                }
                            }
                        }
                    }
                }
                sum
            }
        };
        cache[i][j] = Some(result);
        result
    }

    fn get(&self, row: i32, column: char) -> i32 {
        let i = row as usize - 1;
        let j = (column as u8 - b'A') as usize;
        self.cache_get(i, j, &mut vec![vec![None; self.mat[0].len()]; self.mat.len()])
    }

    fn sum(&mut self, row: i32, column: char, numbers: Vec<String>) -> i32 {
        let i = row as usize - 1;
        let j = (column as u8 - b'A') as usize;
        self.mat[i][j] = Grid::Sum(numbers.into_iter().map(|x| SumType::from_str(&x)).collect());
        self.cache_get(i, j, &mut vec![vec![None; self.mat[0].len()]; self.mat.len()])
    }
}

fn main() {
    let mut excel = Excel::new(3, 'C');
    excel.set(1, 'A', 2);
    assert_eq!(excel.sum(3, 'C', svec!["A1", "A1:B2"]), 4);
    excel.set(2, 'B', 2);
    excel.get(3, 'C');
}
