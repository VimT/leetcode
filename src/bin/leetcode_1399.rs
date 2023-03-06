//! 统计最大组的数目

pub fn count_largest_group(n: i32) -> i32 {
    let mut cnt = [0; 36];
    for mut i in 1..=n {
        let mut c = 0;
        while i > 0 {
            c += i % 10;
            i /= 10;
        }
        cnt[c as usize] += 1;
    }
    let mut result = 0;
    let mut max = 0;
    for num in cnt {
        if num > max {
            max = num;
            result = 1;
        } else if num == max {
            result += 1;
        }
    }
    dbg!()
    result
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(13), 4);
        assert_eq!(func(2), 2);
    }
    test(count_largest_group);
}
