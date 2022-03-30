//! 蜡烛之间的盘子


pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let s = s.as_bytes();
    let len = s.len();
    let mut lazhu = Vec::with_capacity(len);
    for i in 0..len {
        if s[i] == b'|' {
            lazhu.push(i);
        }
    }
    let mut result = vec![0; queries.len()];
    for i in 0..queries.len() {
        let lazhu_left = lazhu.binary_search(&(queries[i][0] as usize)).unwrap_or_else(|x| x);
        if lazhu_left >= lazhu.len() {
            continue;
        }
        let left = lazhu[lazhu_left];
        let lazhu_right = match lazhu.binary_search(&(queries[i][1] as usize)) {
            Ok(v) => v,
            Err(v) => {
                if v == 0 {
                    continue;
                }
                v - 1
            }
        };
        let right = lazhu[lazhu_right];
        if right > left {
            result[i] = (right - left - (lazhu_right - lazhu_left)) as i32;
        }
    }
    result
}

fn main() {
    assert_eq!(plates_between_candles(String::from("******|"), vec![vec![2, 5], vec![5, 9]]), vec![0, 0]);
    assert_eq!(plates_between_candles(String::from("**|**|***|"), vec![vec![2, 5], vec![5, 9]]), vec![2, 3]);
    assert_eq!(plates_between_candles(String::from("***|**|*****|**||**|*"), vec![vec![1, 17], vec![4, 5], vec![14, 17], vec![5, 11], vec![15, 16]]), vec![9, 0, 0, 0, 0]);
}
