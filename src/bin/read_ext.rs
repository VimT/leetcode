use std::fmt::Debug;
use std::io;
use std::io::Stdin;
use std::str::FromStr;

trait ReadExt: io::Read {
    fn read_string(&self) -> String;
    fn read_line_num<T>(&self) -> Result<T, T::Err> where T: FromStr, <T as FromStr>::Err: Debug;
    fn read_vec<T>(&self) -> Vec<T> where T: FromStr, <T as FromStr>::Err: Debug;
}

impl ReadExt for Stdin {
    fn read_string(&self) -> String {
        let mut line = String::new();
        self.read_line(&mut line).unwrap();
        line.trim().to_string()
    }

    fn read_line_num<T>(&self) -> Result<T, T::Err> where T: FromStr, <T as FromStr>::Err: Debug {
        self.read_string().parse::<T>()
    }

    fn read_vec<T>(&self) -> Vec<T> where T: FromStr, <T as FromStr>::Err: Debug {
        self.read_string().split(" ").filter(|x| !x.is_empty()).map(|x| x.parse::<T>().unwrap()).collect()
    }
}

fn main() {
    let stdin = io::stdin();
    let nm: Vec<usize> = stdin.read_vec();
    let target = nm[1] as i32;
    let lines: Vec<i32> = stdin.read_vec();
    let mut left = 1;
    let mut right = *lines.iter().max().unwrap();
    while left < right {
        let mid = (left + right) / 2;
        let mut cnt = 0;
        for &line in &lines {
            cnt += line / mid;
        }
        if cnt >= target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    println!("{}", left - 1);
}
