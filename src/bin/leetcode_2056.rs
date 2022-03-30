//! 棋盘上有效移动组合的数目

pub fn count_combinations(pieces: Vec<String>, mut positions: Vec<Vec<i32>>) -> i32 {
    static dx: [i32; 8] = [1, -1, 0, 0, 1, -1, -1, 1];
    static dy: [i32; 8] = [0, 0, 1, -1, 1, -1, 1, -1];
    for i in 0..positions.len() {
        positions[i][0] -= 1;
        positions[i][1] -= 1;
    }
    // 移动距离。方向
    let mut m = vec![(0, 0); 4];
    // 校验
    fn sim(positions: &Vec<Vec<i32>>, m: &Vec<(usize, i32)>) -> i32 {
        let len = positions.len();
        let mut board = vec![vec![0; 8]; 8];
        let mut mov = vec![(0, 0); 4];
        let mut curpos = vec![vec![0; 2]; 4];
        for i in 0..len {
            mov[i] = m[i];
            curpos[i][0] = positions[i][0];
            curpos[i][1] = positions[i][1];
        }
        loop {
            let mut moved = false;
            for i in 0..len {
                if mov[i].1 > 0 {
                    moved = true;
                    mov[i].1 -= 1;
                    curpos[i][0] += dx[mov[i].0];
                    curpos[i][1] += dy[mov[i].0];
                }
            }
            if !moved { return 1; }
            for i in 0..len {
                board[curpos[i][0] as usize][curpos[i][1] as usize] += 1;
                if board[curpos[i][0] as usize][curpos[i][1] as usize] > 1 {
                    return 0;
                }
            }
            for i in 0..len {
                board[curpos[i][0] as usize][curpos[i][1] as usize] = 0;
            }
        }
    }
    let mut res = 0;
    fn dfs(pieces: &Vec<String>, positions: &Vec<Vec<i32>>, m: &mut Vec<(usize, i32)>, i: usize, res: &mut i32) {
        if i == pieces.len() {
            *res += sim(positions, m);
            return;
        }
        let (min, max) = match pieces[i].as_str() {
            "rook" => (0, 3),
            "queen" => (0, 7),
            "bishop" => (4, 7),
            _ => { panic!() }
        };
        for d in min..=max {
            for l in 1..=8 {
                let x = positions[i][0] + l * dx[d];
                let y = positions[i][1] + l * dy[d];
                if x >= 0 && x < 8 && y >= 0 && y < 8 {
                    m[i].0 = d;
                    m[i].1 = l;
                    dfs(pieces, positions, m, i + 1, res)
                }
            }
        }
        m[i].0 = 0;
        m[i].1 = 0;
        dfs(pieces, positions, m, i + 1, res);
    }

    dfs(&pieces, &positions, &mut m, 0, &mut res);
    res
}

fn main() {
    assert_eq!(count_combinations(vec!["rook"].iter().map(|x| x.to_string()).collect(), vec![vec![1, 1]]), 15);
    assert_eq!(count_combinations(vec!["queen"].iter().map(|x| x.to_string()).collect(), vec![vec![1, 1]]), 22);
    assert_eq!(count_combinations(vec!["bishop"].iter().map(|x| x.to_string()).collect(), vec![vec![4, 3]]), 12);
    assert_eq!(count_combinations(vec!["rook", "rook"].iter().map(|x| x.to_string()).collect(), vec![vec![1, 1], vec![8, 8]]), 223);
    assert_eq!(count_combinations(vec!["queen", "bishop"].iter().map(|x| x.to_string()).collect(), vec![vec![5, 7], vec![3, 4]]), 281);
}