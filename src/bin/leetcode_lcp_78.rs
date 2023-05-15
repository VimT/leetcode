//! 城墙防线

pub fn rampart_defensive_line(rampart: Vec<Vec<i32>>) -> i32 {
    let mut left = 0;
    let len = rampart.len();
    let mut right = rampart[len - 1][0] - rampart[0][1] - rampart[1..len - 1].iter().map(|x| x[1] - x[0]).sum::<i32>();
    right /= len as i32 - 2;
    while left < right {
        let mid = (left + right + 1) / 2;
        let mut ok = true;
        let mut pre_right = rampart[0][1];
        for i in 1..len - 1 {
            let mut r = rampart[i][1];
            let right_need = mid - rampart[i][0] + pre_right;
            if right_need > 0 {
                r += right_need;
            }
            if r > rampart[i + 1][0] {
                ok = false;
                break;
            }
            pre_right = r;
        }
        if ok {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    left
}

fn main() {
    fn test(func: fn(rampart: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(vec![vec![3, 5], vec![12, 29], vec![31, 38], vec![39, 42], vec![43, 44], vec![46, 47]]), 2);
        assert_eq!(func(vec![vec![0, 3], vec![4, 5], vec![7, 9]]), 3);
        assert_eq!(func(vec![vec![1, 2], vec![5, 8], vec![11, 15], vec![18, 25]]), 4);
    }
    test(rampart_defensive_line);
}
