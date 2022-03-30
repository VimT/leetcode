//!  行星碰撞

pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut s = vec![];
    for i in 0..asteroids.len() {
        if asteroids[i] > 0 {
            s.push(asteroids[i]);
        } else {
            let mut need = true;
            while !s.is_empty() && *s.last().unwrap() > 0 {
                let last = *s.last().unwrap();
                if last > -asteroids[i] {
                    need = false;
                    break;
                } else if last == -asteroids[i] {
                    need = false;
                    s.pop();
                    break;
                } else {
                    need = true;
                    s.pop();
                }
            }
            if need {
                s.push(asteroids[i]);
            }
        }
    }
    s
}

fn main() {
    assert_eq!(asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
    assert_eq!(asteroid_collision(vec![8, -8]), vec![]);
    assert_eq!(asteroid_collision(vec![10, 2, -5]), vec![10]);
    assert_eq!(asteroid_collision(vec![-2, -1, 1, 2]), vec![-2, -1, 1, 2]);
}