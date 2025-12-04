use std::fs;

fn part1() {
    let mut answer = 0;
    let mut dial = 50;
    let buffer = fs::read_to_string("input.txt").unwrap();
    for line in buffer.lines() {
        let mut iter = line.chars();
        let dir = iter.next().unwrap();
        let step = iter.as_str().parse::<i32>().unwrap();
        let mul = if dir == 'R' { 1 } else { -1 };
        dial = ((dial + mul * step) % 100 + 100) % 100;
        answer += (dial == 0) as i32;
    }
    println!("Part 1 {}", answer);
}

fn part2() {
    let mut answer = 0;
    let mut dial = 50;
    let buffer = fs::read_to_string("input.txt").unwrap();
    for line in buffer.lines() {
        let mut iter = line.chars();
        let dir = iter.next().unwrap();
        let step = iter.as_str().parse::<i32>().unwrap();
        let prev_dial = dial;
        let mul = if dir == 'R' { 1 } else { -1 };
        dial = ((dial + mul * step) % 100 + 100) % 100;
        let diff = mul * (dial - prev_dial) * prev_dial;
        answer += step / 100 + (diff < 0 || dial == 0) as i32
    }
    println!("Part 2 {}", answer);
}

fn main() {
    part1();
    part2();
}
