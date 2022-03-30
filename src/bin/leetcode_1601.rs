//! 最多可达成的换楼请求数目

pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
    fn dfs(cur: &mut Vec<i32>, requests: &Vec<Vec<i32>>, i: usize, num: i32, result: &mut i32) {
        if i == requests.len() {
            let mut ok = true;
            for num in cur {
                if *num != 0 {
                    ok = false;
                    break;
                }
            }
            if ok {
                *result = (*result).max(num);
            }
            return;
        }
        cur[requests[i][0] as usize] -= 1;
        cur[requests[i][1] as usize] += 1;
        dfs(cur, requests, i + 1, num + 1, result);
        cur[requests[i][0] as usize] += 1;
        cur[requests[i][1] as usize] -= 1;
        dfs(cur, requests, i + 1, num, result)
    }
    let mut result = 0;
    dfs(&mut vec![0; n as usize], &requests, 0, 0, &mut result);
    result
}

pub fn maximum_requests_bit(n: i32, requests: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    let len = requests.len();
    let mut delta = vec![0; n as usize];
    for mask in 0u32..1 << len {
        delta.fill(0);
        let cnt = mask.count_ones();
        if cnt < result {
            continue;
        }
        for (i, req) in requests.iter().enumerate() {
            if mask >> i & 1 == 1 {
                delta[req[0] as usize] += 1;
                delta[req[1] as usize] -= 1;
            }
        }
        let mut ok = true;
        for &num in &delta {
            if num != 0 {
                ok = false;
                break;
            }
        }
        if ok {
            result = result.max(cnt);
        }
    }
    result as i32
}

fn main() {
    assert_eq!(maximum_requests_bit(5, vec![vec![0, 1], vec![1, 0], vec![0, 1], vec![1, 2], vec![2, 0], vec![3, 4]]), 5);
    assert_eq!(maximum_requests_bit(3, vec![vec![0, 0], vec![1, 2], vec![2, 1]]), 3);
    assert_eq!(maximum_requests_bit(4, vec![vec![0, 3], vec![3, 1], vec![1, 2], vec![2, 0]]), 4);
}
