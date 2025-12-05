use std::{fs, path::Path};

fn main() {
    let lines = load_solution("input.txt");
    let (pass, pass2) = solve(lines);
    println!("Star 1: {pass}");
    println!("Star 2: {}", pass + pass2);
}

fn load_solution(file_name: &str) -> Vec<String> {
    let lines = fs::read_to_string(Path::new(file_name)).unwrap();
    lines.lines().map(|s| s.to_string()).collect()
}

enum Rotation {
    Left(i32),
    Right(i32),
}

impl From<String> for Rotation {
    fn from(value: String) -> Self {
        let num = value[1..].parse::<i32>().unwrap();

        match &value[..1] {
            "R" => Self::Right(num),
            "L" => Self::Left(num),
            _ => panic!(),
        }
    }
}

struct Dial {
    state: u32,
    pointed_at_zero: u32,
    exactly_zero: u32,
}

impl Dial {
    fn new() -> Dial {
        Self {
            state: 50,
            pointed_at_zero: 0,
            exactly_zero: 0,
        }
    }

    fn make_turn(&mut self, rotation: Rotation) {
        let mut made_turn = false;

        match rotation {
            Rotation::Left(mut left) => {
                while left > 0 {
                    if self.state == 0 {
                        self.state = 99;
                        if made_turn {
                            self.pointed_at_zero += 1;
                        }
                    } else {
                        self.state -= 1;
                    }

                    left -= 1;
                    made_turn = true;
                }
            }
            Rotation::Right(mut right) => {
                while right > 0 {
                    if self.state == 99 {
                        self.state = 0;
                        if right > 1 {
                            self.pointed_at_zero += 1;
                        }
                    } else {
                        self.state += 1;
                    }

                    right -= 1;
                }
            }
        }

        if self.state == 0 {
            self.exactly_zero += 1;
        }
    }

    fn get_result(self) -> (u32, u32) {
        (self.exactly_zero, self.pointed_at_zero)
    }
}

fn solve(lines: Vec<String>) -> (u32, u32) {
    let mut dial = Dial::new();

    lines
        .into_iter()
        .map(|l| Rotation::from(l))
        .for_each(|rotation| {
            dial.make_turn(rotation);
        });

    dial.get_result()
}
