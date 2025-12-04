use std::fs;

fn part1() {
    let mut sum: u64 = 0;
    let buffer = fs::read_to_string("input.txt").unwrap();
    for line in buffer.split(',') {
        let mut iter = line.split('-');
        let start = iter.next().unwrap().parse::<u64>().unwrap();
        let end = iter.next().unwrap().parse::<u64>().unwrap();
        for i in start..=end {
            let num = i.to_string();
            if num[0..(num.len() / 2)] == num[(num.len() / 2)..] {
                sum += i;
            }
        }
    }
    println!("Part 1 {}", sum);
}

fn part2() {
    let mut sum: u64 = 0;
    let buffer = fs::read_to_string("input.txt").unwrap();
    for line in buffer.split(',') {
        let mut iter = line.split('-');
        let start = iter.next().unwrap().parse::<u64>().unwrap();
        let end = iter.next().unwrap().parse::<u64>().unwrap();
        for num in start..=end {
            let num_str = num.to_string();
            let len = num_str.len();
            'outer_loop: for chunk in 1..=len / 2 {
                if len % chunk != 0 {
                    continue;
                };
                let first = &num_str[0..chunk];
                for i in 1..len / chunk {
                    if *first != num_str[(i * chunk)..(i * chunk + chunk)] {
                        break;
                    }
                    if i == len / chunk - 1 {
                        sum += num;
                        break 'outer_loop;
                    }
                }
            }
        }
    }
    println!("Part 2 {}", sum);
}

fn main() {
    part1();
    part2();
}
