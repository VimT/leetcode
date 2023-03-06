//! 安排电影院座位

pub fn max_number_of_families(mut n: i32, mut reserved_seats: Vec<Vec<i32>>) -> i32 {
    reserved_seats.sort_unstable();
    let mut i = 0;
    let len = reserved_seats.len();
    let mut result = 0;
    while i < len {
        let line = reserved_seats[i][0];
        let mut seat = vec![true; 11];
        seat[reserved_seats[i][1] as usize] = false;
        let mut j = i + 1;
        while j < len && reserved_seats[j][0] == line {
            seat[reserved_seats[j][1] as usize] = false;
            j += 1;
        }
        let a = seat[2..6].iter().all(|&x| x);
        let b = seat[4..8].iter().all(|&x| x);
        let c = seat[6..10].iter().all(|&x| x);
        if a & c {
            result += 2;
        } else if a || b || c {
            result += 1;
        }
        i = j;
        n -= 1;
    }
    result + n * 2
}

fn main() {
    fn test(func: fn(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(3, vec![vec![1, 2], vec![1, 3], vec![1, 8], vec![2, 6], vec![3, 1], vec![3, 10]]), 4);
        assert_eq!(func(2, vec![vec![2, 1], vec![1, 8], vec![2, 6]]), 2);
        assert_eq!(func(4, vec![vec![4, 3], vec![1, 4], vec![4, 6], vec![1, 7]]), 4);
    }
    test(max_number_of_families);
}
