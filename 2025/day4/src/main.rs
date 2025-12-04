use std::fs;

fn is_active(array: &Vec<Vec<char>>, i: i32, j: i32) -> bool {
    let row = (*array).get(i as usize);
    match row {
        Some(x) => match (*x).get(j as usize) {
            Some(y) => *y == '@',
            None => false,
        },
        None => false,
    }
}

fn surrounding_active_count(array: &Vec<Vec<char>>, i: i32, j: i32) -> i32 {
    let mut around = 0;
    if is_active(&array, i - 1, j - 1) {
        around += 1;
    }
    if is_active(&array, i - 1, j) {
        around += 1;
    }
    if is_active(&array, i - 1, j + 1) {
        around += 1;
    }
    if is_active(&array, i, j - 1) {
        around += 1;
    }
    if is_active(&array, i, j + 1) {
        around += 1;
    }
    if is_active(&array, i + 1, j - 1) {
        around += 1;
    }
    if is_active(&array, i + 1, j) {
        around += 1;
    }
    if is_active(&array, i + 1, j + 1) {
        around += 1;
    }
    return around;
}

fn part1() {
    let mut answer = 0;
    let buffer = fs::read_to_string("input.txt").unwrap();
    let array: Vec<Vec<char>> = buffer.lines().map(|f| f.chars().collect()).collect();

    for i in 0..array.len() {
        for j in 0..array[i].len() {
            let cell = array[i][j];
            if cell == '.' {
                continue;
            };
            if surrounding_active_count(&array, i as i32, j as i32) < 4 {
                answer += 1
            }
        }
    }
    println!("Part 1 {}", answer);
}

fn part2() {
    let mut answer = 0;
    let buffer = fs::read_to_string("input.txt").unwrap();
    let mut array: Vec<Vec<char>> = buffer.lines().map(|f| f.chars().collect()).collect();

    loop {
        let old_found = answer;
        for i in 0..array.len() {
            for j in 0..array[i].len() {
                let cell = array[i][j];
                if cell == '.' {
                    continue;
                }
                if surrounding_active_count(&array, i as i32, j as i32) < 4 {
                    answer += 1;
                    array[i][j] = '.';
                }
            }
        }
        if old_found == answer {
            break;
        }
    }
    println!("Part 2 {}", answer);
}

fn main() {
    part1();
    part2();
}
