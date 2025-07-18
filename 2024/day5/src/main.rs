use std::fs;

fn part1() {
    let buffer = fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;

    let mut sort_rules: Vec<(i32, i32)> = Vec::new();
    let mut first_pass = true;
    for line in buffer.lines() {
        if line.len() == 0 {
            first_pass = false;
            continue;
        }
        if first_pass {
            let mut iter = line.split("|");
            let left = iter.next().unwrap().parse::<i32>().unwrap();
            let right = iter.next().unwrap().parse::<i32>().unwrap();
            sort_rules.push((left, right));
        } else {
            let records = line.split(",").collect::<Vec<&str>>();
            let mut correct = true;
            for i in 0..(records.len() - 1) {
                match sort_rules.iter().find(|x| {
                    x.0 == records[i].parse::<i32>().unwrap()
                        && x.1 == records[i + 1].parse::<i32>().unwrap()
                }) {
                    Some(_) => (),
                    None => correct = false,
                }
            }
            if correct {
                sum += records[(records.len() - 1) / 2].parse::<i32>().unwrap();
            }
        }
    }
    println!("Part 1: {sum}")
}

fn part2() {
    let buffer = fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;

    let mut sort_rules: Vec<(&str, &str)> = Vec::new();
    let mut first_pass = true;
    for line in buffer.lines() {
        if line.len() == 0 {
            first_pass = false;
            continue;
        }
        if first_pass {
            let mut iter = line.split("|");
            let left = iter.next().unwrap();
            let right = iter.next().unwrap();
            sort_rules.push((left, right));
        } else {
            let mut records = line.split(",").collect::<Vec<&str>>();
            let mut correct = true;
            for i in 0..(records.len() - 1) {
                match sort_rules
                    .iter()
                    .find(|x| x.0 == records[i] && x.1 == records[i + 1])
                {
                    Some(_) => (),
                    None => correct = false,
                }
            }
            if !correct {
                for i in 0..(records.len() - 1) {
                    for j in 0..(records.len() - i - 1) {
                        match sort_rules
                            .iter()
                            .find(|x| x.0 == records[j] && x.1 == records[j + 1])
                        {
                            Some(_) => {}
                            None => {
                                let temp = records[j];
                                records[j] = records[j + 1];
                                records[j + 1] = temp;
                            }
                        }
                    }
                }
                sum += records[(records.len() - 1) / 2].parse::<i32>().unwrap();
            }
        }
    }
    println!("Part 2: {sum}")
}

fn main() {
    part1();
    part2()
}
