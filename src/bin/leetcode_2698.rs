//! 求一个整数的惩罚数

fn _calc() {
    fn dfs(num: i32, s: &String, i: usize, cur_sum: i32) -> bool {
        if i == s.len() {
            return cur_sum == num;
        }
        for j in i + 1..=s.len() {
            if dfs(num, s, j, cur_sum + s[i..j].parse::<i32>().unwrap()) { return true; }
        }
        false
    }
    let mut result = vec![];
    for i in 1..=1000 {
        let x = i * i;
        if dfs(i, &x.to_string(), 0, 0) {
            result.push(i);
        }
    }
    println!("{:?}", result);
}

pub fn punishment_number(n: i32) -> i32 {
    static CAN_USE: [i32; 29] = [1, 9, 10, 36, 45, 55, 82, 91, 99, 100, 235, 297, 369, 370, 379, 414, 657, 675, 703, 756, 792, 909, 918, 945, 964, 990, 991, 999, 1000];
    let mut result = 0;
    for num in CAN_USE {
        if num > n { break; }
        result += num * num;
    }
    result
}

fn main() {
    fn test(func: fn(n: i32) -> i32) {
        assert_eq!(func(10), 182);
        assert_eq!(func(37), 1478);
    }
    test(punishment_number);
}
