use std::fs;

fn part1() {
    let buffer = fs::read_to_string("input.txt").unwrap();

    let mut sum = 0;
    for line in buffer.lines() {
        let vecs = line
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let prev_diff = vecs[0] - vecs[1];
        for i in 0..(vecs.len() - 1) {
            let diff = vecs[i] - vecs[i + 1];
            if diff.abs() < 1 || diff.abs() > 3 || prev_diff / prev_diff.abs() != diff / diff.abs()
            {
                break;
            }
            if i == vecs.len() - 2 {
                sum += 1;
            }
        }
    }
    println!("Part 1: {sum}")
}

fn part2() {
    let buffer = fs::read_to_string("input.txt").unwrap();

    let mut sum = 0;
    for line in buffer.lines() {
        let vecs = line
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut prev_diff = vecs[0] - vecs[1];
        let mut fault_tolerance = 0;
        for i in 0..(vecs.len() - 1) {
            let diff: i32 = vecs[i] - vecs[i + 1];
            if prev_diff == 0 {
                prev_diff = diff;
            }
            if diff.abs() < 1 || diff.abs() > 3 || prev_diff / prev_diff.abs() != diff / diff.abs()
            {
                fault_tolerance += 1;
                if fault_tolerance > 1 {
                    break;
                }
            }
            if i == vecs.len() - 2 {
                sum += 1;
            }
        }
    }
    println!("Part 2: {sum}")
}

fn main() {
    part1();
    part2()
}
