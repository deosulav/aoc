use std::{
    cmp::{max, min},
    fs,
    ops::RangeInclusive,
};

fn part1() {
    let mut sum: u64 = 0;
    let buffer = fs::read_to_string("input.txt").unwrap();
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
    let mut lines = buffer.lines();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let mut iter = line.split('-');
        let start = iter.next().unwrap().parse::<u64>().unwrap();
        let end = iter.next().unwrap().parse::<u64>().unwrap();
        ranges.push(start..=end);
    }
    for line in lines {
        let start = line.parse::<u64>().unwrap();
        for range in &ranges {
            if range.contains(&start) {
                sum += 1;
                break;
            }
        }
    }
    println!("Part 1 {}", sum);
}

fn part2() {
    let mut sum: u64 = 0;
    let buffer = fs::read_to_string("input.txt").unwrap();
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
    let mut lines = buffer.lines();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let mut iter = line.split('-');
        let start = iter.next().unwrap().parse::<u64>().unwrap();
        let end = iter.next().unwrap().parse::<u64>().unwrap();
        ranges.push(start..=end);
    }

    ranges.sort_by(|a, b| a.start().cmp(b.start()));
    let mut max_seen = 0;
    for range in ranges {
        let mut start = *range.start();
        let end = *range.end();
        start = min(max(start, max_seen + 1), end);
        if end > max_seen {
            sum += end - start + 1;
            max_seen = end;
        }
    }
    println!("Part 2 {}", sum);
}

fn main() {
    part1();
    part2();
}
