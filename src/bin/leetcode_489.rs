//! 扫地机器人


use std::collections::HashSet;

static DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

struct Robot {
    room: Vec<Vec<i32>>,
    x: i32,
    y: i32,
    dir: usize,
}

impl Robot {
    fn new(room: Vec<Vec<i32>>, x: i32, y: i32) -> Self {
        Self { room, x, y, dir: 0 }
    }

    fn move_(&mut self) -> bool {
        let (dx, dy) = DIR[self.dir];
        let (nx, ny) = (self.x + dx, self.y + dy);
        if nx < 0 || nx >= self.room.len() as i32 || ny < 0 || ny >= self.room[0].len() as i32 || self.room[nx as usize][ny as usize] == 0 {
            return false;
        }
        self.x = nx;
        self.y = ny;
        true
    }

    fn turn_left(&mut self) {
        self.dir = (self.dir + 3) % 4;
    }

    fn turn_right(&mut self) {
        self.dir = (self.dir + 1) % 4;
    }

    fn clean(&mut self) {
        self.room[self.x as usize][self.y as usize] = 2;
    }

    fn is_ok(&self) -> bool {
        for row in &self.room {
            for &val in row {
                if val == 1 { return false; }
            }
        }
        true
    }
}

fn clean_room(robot: &mut Robot) {
    fn dfs(bot: &mut Robot, x: i32, y: i32, dir: usize, vis: &mut HashSet<(i32, i32)>) {
        bot.clean();
        for i in 1..=4 {
            bot.turn_right();
            let new_dir = (dir + i) % 4;
            let nx = x + DIR[new_dir].0;
            let ny = y + DIR[new_dir].1;
            if !vis.contains(&(nx, ny)) && bot.move_() {
                vis.insert((nx, ny));
                dfs(bot, nx, ny, new_dir, vis);
                bot.turn_right();
                bot.turn_right();
                bot.move_();
                bot.turn_right();
                bot.turn_right();
            }
        }
    }
    dfs(robot, 0, 0, 0, &mut HashSet::new())
}

fn main() {
    fn help(room: Vec<Vec<i32>>, x: i32, y: i32) -> bool {
        let mut robot = Robot::new(room, x, y);
        clean_room(&mut robot);
        robot.is_ok()
    }
    assert_eq!(help(vec![
        vec![1, 1, 1, 1, 1, 0, 1, 1],
        vec![1, 1, 1, 1, 1, 0, 1, 1],
        vec![1, 0, 1, 1, 1, 1, 1, 1],
        vec![0, 0, 0, 1, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 1, 1, 1],
    ], 1, 3), true)
}