use regex::Regex;

pub fn day_one() {
    let (mut left_list, mut right_list) = get_input_data();

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

pub fn day_one_pt_2() {
    let (left_list, right_list) = get_input_data();

    let mut sum = 0;
    for num in left_list {
        let count = right_list.iter().filter(|&x| *x == num).count();
        sum += num * count as i32;
    }

    println!("Day 1 Pt 2: {}", sum);
}

fn get_input_data() -> (Vec<i32>, Vec<i32>) {
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

    (left_list, right_list)
}
