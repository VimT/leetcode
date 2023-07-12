//! 统计不开心的朋友

pub fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut order = vec![vec![0; n]; n];
    for (from, preference) in preferences.into_iter().enumerate() {
        for (v, to) in preference.into_iter().enumerate() {
            order[from][to as usize] = v + 1;
        }
    }
    let mut match0 = vec![0; n];
    for pair in pairs {
        let (x, y) = (pair[0] as usize, pair[1] as usize);
        match0[x] = y;
        match0[y] = x;
    }

    let mut result = 0;
    for x in 0..n {
        let y = match0[x];
        for u in 0..n {
            let v = match0[u];
            if x != u && order[x][u] < order[x][y] && order[u][x] < order[u][v] {
                result += 1;
                break;
            }
        }
    }
    result
}

fn main() {
    fn test(func: fn(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(4, vec![vec![1, 2, 3], vec![3, 2, 0], vec![3, 1, 0], vec![1, 2, 0]], vec![vec![0, 1], vec![2, 3]]), 2);
        assert_eq!(func(2, vec![vec![1], vec![0]], vec![vec![1, 0]]), 0);
        assert_eq!(func(4, vec![vec![1, 3, 2], vec![2, 3, 0], vec![1, 3, 0], vec![0, 2, 1]], vec![vec![1, 3], vec![0, 2]]), 4);
    }
    test(unhappy_friends);
}
