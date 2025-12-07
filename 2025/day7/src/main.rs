use std::{collections::HashSet, fs};

fn part1() {
    let mut sum = 0;
    let buffer = fs::read_to_string("input.txt").unwrap();
    let arr: Vec<Vec<char>> = buffer.lines().map(|f| f.chars().collect()).collect();
    let rows = arr.len();
    let cols = arr[0].len();
    let mut active: HashSet<usize> = HashSet::new();
    for x in 0..rows {
        for y in 0..cols {
            if arr[x][y] == 'S' {
                active.insert(y);
            }
            if arr[x][y] == '^' && active.contains(&y) {
                active.remove(&y);
                active.insert(y - 1);
                active.insert(y + 1);
                sum += 1;
            }
        }
    }
    println!("Part 1 {}", sum);
}

fn part2() {
    let buffer = fs::read_to_string("input.txt").unwrap();
    let mut arr: Vec<Vec<i64>> = buffer
        .lines()
        .map(|f| {
            f.chars()
                .map(|c| {
                    if c == 'S' {
                        -1
                    } else if c == '^' {
                        -2
                    } else {
                        0
                    }
                })
                .collect()
        })
        .collect();
    let rows = arr.len();
    let cols = arr[0].len();
    for x in 1..rows {
        for y in 0..cols {
            if arr[x - 1][y] == -1 {
                arr[x][y] = 1;
                continue;
            }
            if arr[x - 1][y] != -2 && arr[x][y] >= 0 {
                arr[x][y] = arr[x - 1][y];
            }
            if y > 0 && arr[x - 1][y - 1] == -2 {
                arr[x][y] += arr[x - 2][y - 1];
            }
            if y < cols - 1 && arr[x - 1][y + 1] == -2 {
                arr[x][y] += arr[x - 2][y + 1];
            }
        }
    }
    let sum = arr[rows - 1].iter().fold(0, |acc, num| acc + num);
    println!("Part 2 {}", sum);
}

fn main() {
    part1();
    part2();
}
