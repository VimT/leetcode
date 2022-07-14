//! 路径总和 IV

pub fn path_sum(nums: Vec<i32>) -> i32 {
    let mut map = vec![vec![10; 20]; 10];
    for num in nums {
        map[(num / 100) as usize][(num % 100 / 10) as usize] = num % 10;
    }
    fn dfs(map: &Vec<Vec<i32>>, level: usize, pos: usize, cur: i32, result: &mut i32) {
        let right = pos * 2;
        let left = right - 1;
        if map[level + 1][left] == 10 && map[level + 1][right] == 10 {
            *result += cur;
            return;
        }
        if map[level + 1][left] != 10 {
            dfs(map, level + 1, left, cur + map[level + 1][left], result);
        }
        if map[level + 1][right] != 10 {
            dfs(map, level + 1, right, cur + map[level + 1][right], result);
        }
    }
    if map[1][1] == 10 { return 0; }
    let mut result = 0;
    dfs(&map, 1, 1, map[1][1], &mut result);
    result
}

fn main() {
    fn test(func: fn(nums: Vec<i32>) -> i32) {
        assert_eq!(func(vec![113, 215, 221]), 12);
        assert_eq!(func(vec![113, 221]), 4);
    }
    test(path_sum);
}
