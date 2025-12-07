use std::fs;

fn part1() {
    let mut sum = 0;
    let buffer = fs::read_to_string("input.txt").unwrap();
    let mut array: Vec<Vec<u64>> = Vec::new();
    let lines: Vec<&str> = buffer.lines().collect();
    for line_i in 0..(lines.len() - 1) {
        let line = lines[line_i];
        let mut temp: Vec<u64> = Vec::new();
        for k in line.split(' ') {
            if let Ok(a) = k.parse::<u64>() {
                temp.push(a);
            }
        }
        array.push(temp);
    }
    let mut curr = 0;
    for tok in lines[lines.len() - 1].split(' ') {
        if tok.is_empty() {
            continue;
        }
        let first = tok.chars().next().unwrap();
        let mut t = 1;
        if first == '+' {
            for iter in 0..array.len() {
                t += array[iter][curr];
            }
            curr += 1;
            t -= 1;
        }
        if first == '*' {
            for iter in 0..array.len() {
                t *= array[iter][curr];
            }
            curr += 1;
        }
        sum += t;
    }

    println!("Part 1 {}", sum);
}

fn part2() {
    let mut sum: u64 = 0;
    let buffer = fs::read_to_string("input.txt").unwrap();
    let array: Vec<Vec<char>> = buffer.lines().map(|l| l.chars().collect()).collect();
    let rows = array.len();
    let cols = array.get(0).unwrap().len();

    let mut num_arr: Vec<u64> = Vec::new();
    for y in (0..cols).rev() {
        let mut num: u64 = 0;
        for x in 0..(rows - 1) {
            if let Some(digit) = array[x][y].to_digit(10) {
                num = num * 10 + digit as u64;
            }
        }
        if num == 0 {
            continue;
        }
        num_arr.push(num);
        let op = array[rows - 1][y];
        if op != ' ' {
            sum += num_arr
                .iter()
                .fold(1, |acc, n| if op == '+' { acc + n } else { acc * n });
            if op == '+' {
                sum -= 1;
            }
            num_arr.clear();
        }
    }

    println!("Part 2 {}", sum);
}

fn main() {
    part1();
    part2();
}
