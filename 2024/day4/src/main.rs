use std::{fs, usize};

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

struct Stepper {
    state_space: Vec<char>,
    curr_state: usize,
    success: bool,
}

impl Stepper {
    pub fn new(state_space: Vec<char>) -> Self {
        Self {
            state_space,
            curr_state: 0,
            success: false,
        }
    }

    fn step(&mut self, token: char) {
        if token == XMAS[self.curr_state] {
            self.curr_state += 1;
            if token == XMAS[3] {
                self.success = true;
            }
        }
    }
}

struct Checker {
    state_space: Vec<char>,
}

impl Checker {
    fn check_direction(
        &self,
        grid: &Vec<Vec<char>>,
        i: i32,
        j: i32,
        i_mul: i32,
        j_mul: i32,
    ) -> bool {
        let mut stepper = Stepper::new(self.state_space.clone());
        for iter in 0..self.state_space.len() {
            let x_iter = i + iter as i32 * i_mul;
            if x_iter < 0 {
                continue;
            };
            if let Some(line) = (*grid).get(x_iter as usize) {
                let y_iter = j + iter as i32 * j_mul;
                if y_iter < 0 {
                    continue;
                }
                if let Some(val) = (*line).get(y_iter as usize) {
                    stepper.step(*val);
                }
            }
        }
        stepper.success
    }

    fn check_cross(&self, grid: &Vec<Vec<char>>, i: i32, j: i32, i_mul: i32, j_mul: i32) -> bool {
        let mut stepper = Stepper::new(self.state_space.clone());
        
        stepper.success
    }
}

fn part1() {
    let buffer = fs::read_to_string("input.txt").unwrap();
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in buffer.lines() {
        let mut temp: Vec<char> = Vec::new();
        for char in line.chars() {
            temp.push(char);
        }
        grid.push(temp);
    }

    let mut sum = 0;

    let checker = Checker {
        state_space: ['X', 'M', 'A', 'S'].to_vec(),
    };

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if checker.check_direction(&grid, i as i32, j as i32, 0, 1) {
                sum += 1;
            }
            if checker.check_direction(&grid, i as i32, j as i32, 0, -1) {
                sum += 1;
            }
            if checker.check_direction(&grid, i as i32, j as i32, 1, 0) {
                sum += 1;
            }
            if checker.check_direction(&grid, i as i32, j as i32, -1, 0) {
                sum += 1;
            }
            if checker.check_direction(&grid, i as i32, j as i32, 1, 1) {
                sum += 1;
            }
            if checker.check_direction(&grid, i as i32, j as i32, 1, -1) {
                sum += 1;
            }
            if checker.check_direction(&grid, i as i32, j as i32, -1, 1) {
                sum += 1;
            }
            if checker.check_direction(&grid, i as i32, j as i32, -1, -1) {
                sum += 1;
            }
        }
    }

    println!("Part 1: {:?}", sum);
}

fn part2() {
    let buffer = fs::read_to_string("input.txt").unwrap();
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in buffer.lines() {
        let mut temp: Vec<char> = Vec::new();
        for char in line.chars() {
            temp.push(char);
        }
        grid.push(temp);
    }

    let mut sum = 0;

    let checker = Checker {
        state_space: ['M', 'A', 'S'].to_vec(),
    };

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if checker.check_direction(&grid, i as i32, j as i32, 0, 1) {
                sum += 1;
            }
            if checker.check_direction(&grid, i as i32, j as i32, 0, -1) {
                sum += 1;
            }
            if checker.check_direction(&grid, i as i32, j as i32, 1, 0) {
                sum += 1;
            }
            if checker.check_direction(&grid, i as i32, j as i32, -1, 0) {
                sum += 1;
            }
            if checker.check_direction(&grid, i as i32, j as i32, 1, 1) {
                sum += 1;
            }
            if checker.check_direction(&grid, i as i32, j as i32, 1, -1) {
                sum += 1;
            }
            if checker.check_direction(&grid, i as i32, j as i32, -1, 1) {
                sum += 1;
            }
            if checker.check_direction(&grid, i as i32, j as i32, -1, -1) {
                sum += 1;
            }
        }
    }

    println!("Part 2: {:?}", sum);
}

fn main() {
    part1();
    part2()
}
