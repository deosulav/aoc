use std::{collections::HashMap, fs};

fn part1() {
    let buffer = fs::read_to_string("input.txt").unwrap();

    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();
    for line in buffer.lines() {
        let mut iter = line.split("   ");
        first_list.push(iter.next().unwrap().parse::<i32>().unwrap());
        second_list.push(iter.next().unwrap().parse::<i32>().unwrap());
    }

    first_list.sort();
    second_list.sort();

    let mut sum = 0;
    for i in 0..first_list.len() {
        sum += (first_list[i] - second_list[i]).abs();
    }
    println!("Part 1: {sum}")
}

fn part2() {
    let buffer = fs::read_to_string("input.txt").unwrap();

    let mut first_list: HashMap<i32, i32> = HashMap::new();
    let mut second_list: HashMap<i32, i32> = HashMap::new();
    for line in buffer.lines() {
        let mut iter = line.split("   ");
        let first_val = iter.next().unwrap().parse::<i32>().unwrap();
        match first_list.get(&first_val) {
            Some(k) => first_list.insert(first_val, k + 1),
            None => first_list.insert(first_val, 1),
        };
        let val = iter.next().unwrap().parse::<i32>().unwrap();
        match second_list.get(&val) {
            Some(k) => second_list.insert(val, k + 1),
            None => second_list.insert(val, 1),
        };
    }

    let mut sum = 0;
    for i in first_list {
        match second_list.get(&i.0) {
            Some(k) => sum += i.0 * i.1 * k,
            None => sum += 0,
        };
    }
    println!("Part 2: {sum}")
}

fn main() {
    part1();
    part2()
}
