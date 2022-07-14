//! 孤独像素 I

pub fn find_lonely_pixel(picture: Vec<Vec<char>>) -> i32 {
    let m = picture.len();
    let n = picture[0].len();
    let mut rows = vec![vec![]; m];
    let mut cols = vec![vec![]; n];
    for i in 0..m {
        for j in 0..n {
            if picture[i][j] == 'B' {
                rows[i].push(j);
                cols[j].push(i);
            }
        }
    }
    let mut result = 0;
    for i in 0..m {
        if rows[i].len() == 1 {
            if cols[rows[i][0]].len() == 1 {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(picture: Vec<Vec<char>>) -> i32) {
        assert_eq!(func(vec![vec!['W', 'W', 'B'], vec!['W', 'B', 'W'], vec!['B', 'W', 'W']]), 3);
        assert_eq!(func(vec![vec!['B', 'B', 'B'], vec!['B', 'B', 'W'], vec!['B', 'B', 'B']]), 0);
    }
    test(find_lonely_pixel);
}
