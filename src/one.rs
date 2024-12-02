use regex::Regex;

pub fn day_one() {
    let file = include_str!("../input/one.txt");
    let lines: Vec<&str> = file.lines().collect();

    let delimiter_regex = Regex::new(r"[^0-9]+").unwrap();
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in lines {
        let split: Vec<&str> = delimiter_regex.split(line).collect();
        left_list.push(split[0].parse::<i32>().unwrap());
        right_list.push(split[1].parse::<i32>().unwrap());
    }

    left_list.sort();
    right_list.sort();

    let mut sum = 0;
    for i in 0..left_list.len() {
        let left = left_list[i];
        let right = right_list[i];
        let dist = (right - left).abs();
        sum += dist;
    }

    println!("Day 1: {}", sum);
}
