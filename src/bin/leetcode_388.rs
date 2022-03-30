//! 文件的最长绝对路径

pub fn length_longest_path(input: String) -> i32 {
    let lines = input.split("\n");
    let mut cur: Vec<&[u8]> = vec![];
    let mut result = 1;
    let mut cur_len = 0;
    for str_line in lines {
        let line = str_line.as_bytes();
        let mut depth = 0;
        while line[depth] == b'\t' { depth += 1; }
        while depth < cur.len() {
            let pop = cur.pop().unwrap();
            cur_len -= pop.len() + 1;
        }
        if depth >= cur.len() {
            let x = &line[depth..];
            cur.push(x);
            cur_len += x.len() + 1;
        }
        if line.contains(&b'.') {
            result = result.max(cur_len);
        }
    }
    result as i32 - 1
}

fn main() {
    assert_eq!(length_longest_path(String::from("dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext")), 20);
    assert_eq!(length_longest_path(String::from("dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext")), 32);
    assert_eq!(length_longest_path(String::from("a")), 0);
    assert_eq!(length_longest_path(String::from("file1.txt\nfile2.txt\nlongfile.txt")), 12);
}
