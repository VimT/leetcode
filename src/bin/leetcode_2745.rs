//! 构造最长的新字符串

const MAX: usize = 51;
static mut MEM: [[[[i32; 3]; MAX]; MAX]; MAX] = [[[[-1;3];MAX];MAX];MAX];

fn dfs(x: usize, y: usize, z: usize, k: usize) -> i32 {
    unsafe {
        if MEM[x][y][z][k] != -1 { return MEM[x][y][z][k]; }
        let mut result = 0;
        if k == 0 {
            if y > 0 { result = result.max(dfs(x, y - 1, z, 1) + 2); }
        } else {
            if x > 0 { result = result.max(dfs(x - 1, y, z, 0) + 2) }
            if z > 0 { result = result.max(dfs(x, y, z - 1, 2) + 2) }
        }
        MEM[x][y][z][k] = result;
        result
    }
}

pub fn longest_string(x: i32, y: i32, z: i32) -> i32 {
    let (x, y, z) = (x as usize, y as usize, z as usize);
    dfs(x, y, z, 0).max(dfs(x, y, z, 1))
}


/// 考虑只有 AA, BB时，AB可以在BB后无限插入
pub fn longest_string2(x: i32, y: i32, z: i32) -> i32 {
    2 * (2 * x.min(y) + (x != y) as i32 + z)
}

fn main() {
    fn test(func: fn(x: i32, y: i32, z: i32) -> i32) {
        assert_eq!(func(2, 5, 1), 12);
        assert_eq!(func(3, 2, 2), 14);
    }
    test(longest_string);
    test(longest_string2);
}
