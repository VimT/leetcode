//! 螺旋矩阵 III

pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
    let mut len = 1;
    let size = (rows * cols) as usize;
    let mut result = Vec::with_capacity(size);
    let mut x = r_start;
    let mut y = c_start;
    result.push(vec![x, y]);
    let result_ptr = &mut result as *mut Vec<Vec<i32>>;
    let push = |x: i32, y: i32| unsafe {
        if x >= 0 && x < rows && y >= 0 && y < cols {
            (*result_ptr).push(vec![x, y]);
        }
    };
    loop {
        let target = y + len;
        if x >= 0 && x < rows {
            while y < target {
                y += 1;
                push(x, y);
            }
        } else {
            y = target;
        }
        if result.len() == size { break; }

        let target = x + len;
        if y >= 0 && y < cols {
            while x < target {
                x += 1;
                push(x, y);
            }
        } else {
            x = target;
        }
        if result.len() == size { break; }
        len += 1;

        let target = y - len;
        if x >= 0 && x < rows {
            while y > target {
                y -= 1;
                push(x, y);
            }
        } else {
            y = target;
        }

        if result.len() == size { break; }
        let target = x - len;
        if y >= 0 && y < cols {
            while x > target {
                x -= 1;
                push(x, y);
            }
        } else {
            x = target;
        }

        if result.len() == size { break; }
        len += 1;
    }
    result
}

fn main() {
    assert_eq!(spiral_matrix_iii(5, 6, 1, 4), vec![vec![1, 4], vec![1, 5], vec![2, 5], vec![2, 4], vec![2, 3], vec![1, 3], vec![0, 3], vec![0, 4], vec![0, 5], vec![3, 5], vec![3, 4], vec![3, 3], vec![3, 2], vec![2, 2], vec![1, 2], vec![0, 2], vec![4, 5], vec![4, 4], vec![4, 3], vec![4, 2], vec![4, 1], vec![3, 1], vec![2, 1], vec![1, 1], vec![0, 1], vec![4, 0], vec![3, 0], vec![2, 0], vec![1, 0], vec![0, 0]]);
    assert_eq!(spiral_matrix_iii(1, 4, 0, 0), vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3]]);
}
