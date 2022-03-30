use std::io;

fn read_string(stdin: &io::Stdin) -> String {
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn read_i32_vec(stdin: &io::Stdin) -> Vec<i32> {
    read_string(stdin).split(" ").map(|x| x.parse::<i32>().unwrap()).collect()
}

fn read_usize_vec(stdin: &io::Stdin) -> Vec<usize> {
    read_string(stdin).split(" ").map(|x| x.parse::<usize>().unwrap()).collect()
}


// 铺地毯 二维差分
fn main() {
    let stdin = io::stdin();
    let line = read_i32_vec(&stdin);
    let n = line[0] as usize;
    let mut a = vec![vec![0; n + 2]; n + 2];
    let m = line[1];
    for _ in 0..m {
        let line = read_usize_vec(&stdin);
        a[line[0]][line[1]] += 1;
        a[line[2] + 1][line[1]] -= 1;
        a[line[0]][line[3] + 1] -= 1;
        a[line[2] + 1][line[3] + 1] += 1;
    }
    for i in 1..=n {
        for j in 1..=n {
            a[i][j] += a[i - 1][j] + a[i][j - 1] - a[i - 1][j - 1];
        }
    }
    for i in 1..=n {
        println!("{}", a[i][1..=n].iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}
