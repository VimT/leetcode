//! 旅客可以得到的最多点数

pub fn max_score(n: i32, k: i32, stay_score: Vec<Vec<i32>>, travel_score: Vec<Vec<i32>>) -> i32 {
    fn dfs(g @ (n, k, stay_score, travel_score): (usize, usize, &Vec<Vec<i32>>, &Vec<Vec<i32>>), i: usize, curr: usize, mem: &mut Vec<Vec<i32>>) -> i32 {
        if i == k { return 0; }
        if mem[i][curr] != -1 { return mem[i][curr]; }
        let mut result = dfs(g, i + 1, curr, mem) + stay_score[i][curr];
        for dest in 0..n {
            result = result.max(dfs(g, i + 1, dest, mem) + travel_score[curr][dest]);
        }
        mem[i][curr] = result;
        result
    }

    let n = n as usize;
    let k = k as usize;
    let mut mem = vec![vec![-1; n]; k];
    (0..n).map(|i| dfs((n, k, &stay_score, &travel_score), 0, i, &mut mem)).max().unwrap()
}

fn main() {
    use leetcode::vvec;
    fn test(func: fn(n: i32, k: i32, stay_score: Vec<Vec<i32>>, travel_score: Vec<Vec<i32>>) -> i32) {
        assert_eq!(func(2, 1, vvec![[2,3]], vvec![[0,2],[1,0]]), 3);
        assert_eq!(func(3, 2, vvec![[3,4,2],[2,1,2]], vvec![[0,2,1],[2,0,4],[3,2,0]]), 8);
    }
    test(max_score);
}
