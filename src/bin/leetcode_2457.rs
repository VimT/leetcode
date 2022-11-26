//! 美丽整数的最小增量

pub fn make_integer_beautiful(n: i64, target: i32) -> i64 {
    let mut v = vec![];
    let mut num = n;
    while num > 0 {
        v.push((num % 10) as i32);
        num /= 10;
    }
    v.push(0);
    v.reverse();
    let mut sum: i32 = v.iter().sum();
    if sum <= target { return 0; }
    let len = v.len();
    sum += 1;
    for i in 0..len - 1 {
        sum -= v[len - i - 1];
        if v[len - i - 2] == 9 { continue; }
        if sum <= target {
            v[len - i - 1..].fill(0);
            v[len - i - 2] += 1;
            let mut num = 0;
            for j in 0..len {
                num = num * 10 + v[j] as i64;
            }
            return num - n;
        }
    }
    unreachable!()
}

fn main() {
    assert_eq!(make_integer_beautiful(94598,6), 5402);
    assert_eq!(make_integer_beautiful(9, 1), 1);
    assert_eq!(make_integer_beautiful(16, 6), 4);
    assert_eq!(make_integer_beautiful(467, 6), 33);
    assert_eq!(make_integer_beautiful(1, 1), 0);
}
