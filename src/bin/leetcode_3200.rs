//! 三角形的最大高度

pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
    fn trytry(mut a: i32, mut b: i32) -> i32 {
        let mut cur = 1;
        let mut is_a = true;
        loop {
            if is_a {
                if a < cur { return cur - 1; }
                a -= cur;
            } else {
                if b < cur { return cur - 1; }
                b -= cur;
            }
            cur += 1;
            is_a = !is_a;
        }
    }
    trytry(red, blue).max(trytry(blue, red))
}

fn main() {
    fn test(func: fn(red: i32, blue: i32) -> i32) {
        assert_eq!(func(1, 1), 1);
        assert_eq!(func(2, 4), 3);
        assert_eq!(func(2, 1), 2);
        assert_eq!(func(10, 1), 2);
    }
    test(max_height_of_triangle);
}
