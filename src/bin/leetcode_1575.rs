//! 统计所有可行路径

pub fn count_routes(mut locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
    const MOD: i32 = 1e9 as i32 + 7;
    fn dfs(loc: &Vec<i32>, finish: usize, cur: usize, left_fuel: i32, mem: &mut Vec<Vec<i32>>) -> i32 {
        if mem[cur][left_fuel as usize] != -1 {
            return mem[cur][left_fuel as usize];
        }
        if left_fuel < (loc[finish] - loc[cur]).abs() { return 0; }
        let mut r = cur + 1;
        let mut result = 0;
        if cur == finish { result += 1; }
        while r < loc.len() && loc[r] - loc[cur] <= left_fuel {
            result = (result + dfs(loc, finish, r, left_fuel - (loc[r] - loc[cur]), mem)) % MOD;
            r += 1;
        }
        let mut l = cur;
        while l > 0 && loc[cur] - loc[l - 1] <= left_fuel {
            result = (result + dfs(loc, finish, l - 1, left_fuel - (loc[cur] - loc[l - 1]), mem)) % MOD;
            l -= 1;
        }
        mem[cur][left_fuel as usize] = result;
        result
    }
    let start = locations[start as usize];
    let finish = locations[finish as usize];
    locations.sort_unstable();
    dfs(&locations, locations.binary_search(&start).unwrap(), locations.binary_search(&finish).unwrap(), fuel, &mut vec![vec![-1; fuel as usize + 1]; locations.len()])
}

/// dfs转dp
pub fn count_routes2(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
    const MOD: i32 = 1e9 as i32 + 7;
    let len = locations.len();
    let fuel = fuel as usize;
    let mut dp = vec![vec![0; fuel + 1]; len];
    for i in 0..=fuel { dp[finish as usize][i] = 1; } // 到终点就有1个方案
    for i in 0..=fuel {
        for j in 0..len {
            for k in 0..len {
                let cost = (locations[j] - locations[k]).abs() as usize;
                if j != k && cost <= i {
                    dp[j][i] = (dp[j][i] + dp[k][i - cost]) % MOD;
                }
            }
        }
    }
    dp[start as usize][fuel]
}

fn main() {
    fn test(func: fn(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32) {
        assert_eq!(func(vec![1, 2, 3], 0, 2, 40), 615088286);
        assert_eq!(func(vec![2, 3, 6, 8, 4], 1, 3, 5), 4);
        assert_eq!(func(vec![4, 3, 1], 1, 0, 6), 5);
        assert_eq!(func(vec![5, 2, 1], 0, 2, 3), 0);
    }
    test(count_routes);
    test(count_routes2);
}
