//! 模拟行走机器人 II

struct Robot {
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    dir: u8,
}

impl Robot {
    fn new(width: i32, height: i32) -> Self {
        Robot { width, height, x: 0, y: 0, dir: 0 }
    }

    fn r#move(&mut self, mut num: i32) {
        let zhou = (self.width + self.height) * 2;
        num %= zhou;
        while num > 0 {
            let mut change_dir = false;
            match self.dir {
                0 => {
                    if num > self.width - 1 - self.x {
                        change_dir = true;
                        num -= self.width - 1 - self.x;
                        self.x = self.width - 1;
                    } else {
                        self.x += num;
                        num = 0;
                    }
                }
                1 => {
                    if num > self.height - 1 - self.y {
                        change_dir = true;
                        num -= self.height - 1 - self.y;
                        self.y = self.height - 1;
                    } else {
                        self.y += num;
                        num = 0;
                    }
                }
                2 => {
                    if num > self.x {
                        change_dir = true;
                        num -= self.x;
                        self.x = 0;
                    } else {
                        self.x -= num;
                        num = 0;
                    }
                }
                3 => {
                    if num > self.y {
                        change_dir = true;
                        num -= self.y;
                        self.y = 0;
                    } else {
                        self.y -= num;
                        num = 0;
                    }
                }
                _ => panic!()
            }
            if change_dir {
                self.dir += 1;
                if self.dir == 4 { self.dir = 0; }
            }
        }
    }

    fn get_pos(&self) -> Vec<i32> {
        vec![self.x, self.y]
    }

    fn get_dir(&self) -> String {
        match self.dir {
            0 => String::from("East"),
            1 => String::from("North"),
            2 => String::from("West"),
            4 => String::from("South"),
            _ => panic!()
        }
    }
}

fn main() {
    let mut robot = Robot::new(6, 3); // 初始化网格图，机器人在 (0, 0) ，朝东。
    robot.r#move(2);  // 机器人朝东移动 2 步，到达 (2, 0) ，并朝东。
    robot.r#move(2);  // 机器人朝东移动 2 步，到达 (4, 0) ，并朝东。
    assert_eq!(robot.get_pos(), vec![4, 0]);
    assert_eq!(robot.get_dir(), String::from("East"));
    robot.r#move(2);  // 朝东移动 1 步到达 (5, 0) ，并朝东。
    // 下一步继续往东移动将出界，所以逆时针转变方向朝北。
    // 然后，往北移动 1 步到达 (5, 1) ，并朝北。
    robot.r#move(1);  // 朝北移动 1 步到达 (5, 2) ，并朝 北 （不是朝西）。
    robot.r#move(4);  // 下一步继续往北移动将出界，所以逆时针转变方向朝西。
    // 然后，移动 4 步到 (1, 2) ，并朝西。
    assert_eq!(robot.get_pos(), vec![1, 2]);
    assert_eq!(robot.get_dir(), String::from("West"));
}
