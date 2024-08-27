//! 矩阵中的蛇

pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
    let mut pos = 0;
    for command in commands {
        match command.as_str() {
            "LEFT" => pos -= 1,
            "RIGHT" => pos += 1,
            "UP" => pos -= n,
            "DOWN" => pos += n,
            _ => unreachable!()
        }
    }
    pos
}

fn main() {
    use leetcode::svec;
    fn test(func: fn(n: i32, commands: Vec<String>) -> i32) {
        assert_eq!(func(2, svec!["RIGHT","DOWN"]), 3);
        assert_eq!(func(3, svec!["DOWN","RIGHT","UP"]), 1);
    }
    test(final_position_of_snake);
}
