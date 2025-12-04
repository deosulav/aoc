use std::fs;

fn part1() {
    let mut sum = 0;
    let buffer = fs::read_to_string("input.txt").unwrap();
    for line in buffer.lines() {
        let mut iter = line.chars();
        let mut first = iter.next().unwrap().to_digit(10).unwrap();
        let mut second = iter.next().unwrap().to_digit(10).unwrap();
        for i in iter {
            let num = i.to_digit(10).unwrap();

            if num > second {
                if second > first {
                    first = second;
                }
                second = num;
            } else {
                if second > first {
                    first = second;
                    second = num;
                }
            }
        }
        sum += first * 10 + second
    }
    println!("Part 1 {}", sum);
}

fn part2() {
    let mut sum: u64 = 0;
    let buffer = fs::read_to_string("input.txt").unwrap();
    let mut perm: String;
    for line in buffer.lines() {
        let mut original = String::from(&line[0..12]);
        let mut high = original.clone();
        for i in line[12..].chars() {
            for j in 0..12 {
                perm = format!("{}{}{}", &original[0..j], &original[j + 1..], i);
                if perm.parse::<u64>().unwrap() > high.parse::<u64>().unwrap() {
                    high = perm.clone();
                }
            }
            original = high.clone();
        }
        sum += high.parse::<u64>().unwrap();
    }
    println!("Part 2 {}", sum);
}

fn max_volt(arr: &[u64], k: usize) -> u64 {
    if k == 1 {
        return *arr.iter().max().unwrap();
    }
    let max_index = arr[0..(arr.len() - k + 1)]
        .iter()
        .enumerate()
        .max_by(|(index1, value1), (index2, value2)| {
            value1.cmp(&value2).then_with(|| index2.cmp(index1))
        })
        .map(|(index, _)| index)
        .unwrap();
    return arr[max_index] * (10u64.pow((k - 1) as u32)) + max_volt(&arr[(max_index + 1)..], k - 1);
}

fn part2_alt() {
    let mut sum: u64 = 0;
    let buffer = fs::read_to_string("input.txt").unwrap();
    for line in buffer.lines() {
        let slice = line
            .chars()
            .map(|a| a.to_digit(10).unwrap() as u64)
            .collect::<Vec<u64>>();
        sum += max_volt(&slice, 12);
    }
    println!("Part 2 {}", sum);
}

fn main() {
    part1();
    part2();
    part2_alt();
}
