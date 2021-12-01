use crate::utils;

pub fn q1() {
    let content = utils::file_to_string("src/inputs/day1.txt");
    let numbers:Vec<i32> = utils::lines_to_int(utils::content_to_lines(&content));

    let mut increases = 0;
    let mut prev:i32 = 0;
    for (i,x) in numbers.iter().enumerate() {
        if i > 0 && x > &prev {
            increases += 1;
        }
        prev = *x
    }
    println!("Day1 Q1: {}", increases)
}

pub fn q2() {
    let content = utils::file_to_string("src/inputs/day1.txt");
    let numbers:Vec<i32> = utils::lines_to_int(utils::content_to_lines(&content));


    let mut increases = 0;
    let group_count = numbers.len() - 2;
    let mut processed_count = 0;
    let mut prev_total = 0;

    loop {
        let current_group = &numbers[processed_count..(processed_count+3)];

        let total = &current_group.iter().fold(0, |mut acc, x| x + acc);
        if processed_count > 0 && total > &prev_total {
            increases += 1;
        }
        prev_total = *total;

        processed_count += 1;
        if processed_count >= group_count {
            break;
        }
    }

    println!("Day1 Q2: {}", increases)
}