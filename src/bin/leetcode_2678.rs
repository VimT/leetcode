//! 老人的数目

pub fn count_seniors(details: Vec<String>) -> i32 {
    details.into_iter().filter(|x| x[11..=12].parse::<i32>().unwrap() > 60).count() as i32
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(details: Vec<String>) -> i32) {
        assert_eq!(func(svec!["7868190130M7522","5303914400F9211","9273338290F4010"]), 2);
        assert_eq!(func(svec!["1313579440F2036","2921522980M5644"]), 0);
    }
    test(count_seniors);
}
