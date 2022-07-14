//! 贪吃蛇

use std::collections::VecDeque;

struct SnakeGame {
    x: i32,
    y: i32,
    food: Vec<Vec<i32>>,
    width: i32,
    height: i32,
    snake: VecDeque<(i32, i32)>,
    score: i32,
}


impl SnakeGame {
    fn new(width: i32, height: i32, mut food: Vec<Vec<i32>>) -> Self {
        let mut q = VecDeque::new();
        food.reverse();
        q.push_back((0, 0));
        Self {
            x: 0,
            y: 0,
            food,
            width,
            height,
            snake: q,
            score: 0,
        }
    }

    fn make_a_move(&mut self, direction: String) -> i32 {
        match direction.as_str() {
            "R" => self.y += 1,
            "D" => self.x += 1,
            "U" => self.x -= 1,
            "L" => self.y -= 1,
            _ => unreachable!()
        }

        if self.x < 0 || self.x >= self.height || self.y < 0 || self.y >= self.width {
            return -1;
        }
        let eat_food = !self.food.is_empty() && *self.food.last().unwrap() == [self.x, self.y];
        if !eat_food {
            self.snake.pop_front();
        }
        for &(sx, sy) in &self.snake {
            if self.x == sx && self.y == sy {
                return -1;
            }
        }
        self.snake.push_back((self.x, self.y));
        if eat_food {
            self.score += 1;
            self.food.pop();
        }
        self.score
    }
}


fn main() {
    let mut sg = SnakeGame::new(3, 2, vec![vec![1, 2], vec![0, 1]]);
    assert_eq!(sg.make_a_move(String::from("R")), 0); // 返回 0
    assert_eq!(sg.make_a_move(String::from("D")), 0); // 返回 0
    assert_eq!(sg.make_a_move(String::from("R")), 1); // 返回 1 ，蛇吃掉了第一个食物，同时第二个食物出现在 (0, 1)
    assert_eq!(sg.make_a_move(String::from("U")), 1); // 返回 1
    assert_eq!(sg.make_a_move(String::from("L")), 2); // 返回 2 ，蛇吃掉了第二个食物，没有出现更多食物
    assert_eq!(sg.make_a_move(String::from("U")), -1); // 返回 -1 ，蛇与边界相撞，游戏结束

    let mut sg = SnakeGame::new(3, 3, vec![vec![2, 0], vec![0, 0], vec![0, 2], vec![0, 1], vec![2, 2], vec![0, 1]]);
    assert_eq!(sg.make_a_move(String::from("D")), 0);
    assert_eq!(sg.make_a_move(String::from("D")), 1);
    assert_eq!(sg.make_a_move(String::from("R")), 1);
    assert_eq!(sg.make_a_move(String::from("U")), 1);
    assert_eq!(sg.make_a_move(String::from("U")), 1);
    assert_eq!(sg.make_a_move(String::from("L")), 2);
    assert_eq!(sg.make_a_move(String::from("D")), 2);
    assert_eq!(sg.make_a_move(String::from("R")), 2);
    assert_eq!(sg.make_a_move(String::from("R")), 2);
    assert_eq!(sg.make_a_move(String::from("U")), 3);
    assert_eq!(sg.make_a_move(String::from("L")), 4);
    assert_eq!(sg.make_a_move(String::from("L")), 4);
    assert_eq!(sg.make_a_move(String::from("D")), 4);
    assert_eq!(sg.make_a_move(String::from("R")), 4);
    assert_eq!(sg.make_a_move(String::from("U")), -1);
}
