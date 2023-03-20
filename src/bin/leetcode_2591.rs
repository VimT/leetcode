//! 将钱分给最多的儿童

pub fn dist_money(money: i32, children: i32) -> i32 {
    for i in (0..=(money / 8).min(children)).rev() {
        let left_children = children - i;
        let left_money = money - i * 8;
        if left_money >= left_children {
            if !(left_children == 1 && left_money == 4) && !(left_children == 0 && left_money > 0) {
                return i;
            }
        }
    }
    -1
}

fn main() {
    fn test(func: fn(money: i32, children: i32) -> i32) {
        assert_eq!(func(24, 2), 1);
        assert_eq!(func(17, 2), 1);
        assert_eq!(func(1, 2), -1);
        assert_eq!(func(20, 3), 1);
        assert_eq!(func(16, 2), 2);
    }
    test(dist_money);
}
