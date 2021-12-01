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